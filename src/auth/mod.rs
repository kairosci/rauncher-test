use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub account_id: String,
}

impl AuthToken {
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn save(&self) -> Result<()> {
        let auth_path = Self::auth_path()?;

        if let Some(parent) = auth_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize token to JSON
        let json_data = serde_json::to_string(self)?;

        // Encrypt the token data using simple XOR encryption with system-derived key
        // Note: For production use, consider OS keychain/credential manager
        let encrypted_data = Self::encrypt_data(json_data.as_bytes())?;

        // Encode as base64 for storage
        let encoded = general_purpose::STANDARD.encode(&encrypted_data);
        fs::write(&auth_path, encoded)?;

        // Set restrictive file permissions (0600) on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&auth_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&auth_path, perms)?;
        }

        Ok(())
    }

    pub fn load() -> Result<Option<Self>> {
        let auth_path = Self::auth_path()?;

        if !auth_path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&auth_path)?;

        // Try to load as encrypted data first (new format)
        match Self::load_encrypted(&contents) {
            Ok(token) => Ok(Some(token)),
            Err(_) => {
                // Fall back to trying plain JSON (old format for migration)
                log::info!("Attempting to migrate from old token format");
                match serde_json::from_str::<AuthToken>(&contents) {
                    Ok(token) => {
                        // Successfully loaded old format, re-save in encrypted format
                        log::info!("Migrating token to encrypted format");
                        token.save()?;
                        Ok(Some(token))
                    }
                    Err(e) => {
                        log::error!("Failed to load token in any format: {}", e);
                        Err(Error::Auth(
                            "Failed to load authentication token".to_string(),
                        ))
                    }
                }
            }
        }
    }

    fn load_encrypted(encoded: &str) -> Result<Self> {
        // Decode from base64
        let encrypted_data = general_purpose::STANDARD
            .decode(encoded.trim())
            .map_err(|e| Error::Auth(format!("Failed to decode token: {}", e)))?;

        // Decrypt the data
        let decrypted_data = Self::decrypt_data(&encrypted_data)?;

        // Parse JSON
        let json_str = String::from_utf8(decrypted_data)
            .map_err(|e| Error::Auth(format!("Invalid token data: {}", e)))?;
        let token: AuthToken = serde_json::from_str(&json_str)
            .map_err(|e| Error::Auth(format!("Failed to parse token: {}", e)))?;

        Ok(token)
    }

    pub fn delete() -> Result<()> {
        let auth_path = Self::auth_path()?;

        if auth_path.exists() {
            fs::remove_file(&auth_path)?;
        }

        Ok(())
    }

    fn auth_path() -> Result<PathBuf> {
        let data_dir = Config::data_dir()?;
        Ok(data_dir.join("auth.json"))
    }

    /// Get encryption key derived from system information
    /// Note: This is a basic implementation. For production use, consider:
    /// - OS keychain/credential manager (keyring crate)
    /// - Hardware-backed key storage
    /// - User password-derived keys (with proper KDF)
    fn get_encryption_key() -> Result<Vec<u8>> {
        // Derive a key from system-specific information
        // This provides obfuscation rather than strong encryption
        let mut hasher = Sha256::new();

        // Add system-specific entropy
        #[cfg(target_os = "linux")]
        {
            // Use machine-id on Linux if available
            if let Ok(machine_id) = fs::read_to_string("/etc/machine-id") {
                hasher.update(machine_id.trim().as_bytes());
            } else if let Ok(machine_id) = fs::read_to_string("/var/lib/dbus/machine-id") {
                hasher.update(machine_id.trim().as_bytes());
            }
        }

        // Add application-specific constant
        hasher.update(b"r-games-launcher-auth-key-v1");

        // Add username for per-user encryption
        if let Ok(username) = std::env::var("USER") {
            hasher.update(username.as_bytes());
        }

        Ok(hasher.finalize().to_vec())
    }

    /// Encrypt data using XOR cipher with derived key
    fn encrypt_data(data: &[u8]) -> Result<Vec<u8>> {
        let key = Self::get_encryption_key()?;
        let mut encrypted = Vec::with_capacity(data.len());

        for (i, byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }

        Ok(encrypted)
    }

    /// Decrypt data using XOR cipher with derived key
    fn decrypt_data(data: &[u8]) -> Result<Vec<u8>> {
        // XOR is symmetric, so decrypt is the same as encrypt
        Self::encrypt_data(data)
    }
}

#[derive(Clone)]
pub struct AuthManager {
    token: Option<AuthToken>,
}

impl AuthManager {
    pub fn new() -> Result<Self> {
        let token = AuthToken::load()?;
        Ok(Self { token })
    }

    pub fn is_authenticated(&self) -> bool {
        if let Some(token) = &self.token {
            !token.is_expired()
        } else {
            false
        }
    }

    pub fn get_token(&self) -> Result<&AuthToken> {
        match &self.token {
            Some(token) if !token.is_expired() => Ok(token),
            _ => Err(Error::NotAuthenticated),
        }
    }

    /// Check if token will expire soon (within 5 minutes)
    pub fn token_needs_refresh(&self) -> bool {
        if let Some(token) = &self.token {
            let now = chrono::Utc::now();
            let time_until_expiry = token.expires_at.signed_duration_since(now);
            time_until_expiry.num_minutes() < 5
        } else {
            false
        }
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        self.token.as_ref().map(|t| t.refresh_token.clone())
    }

    pub fn set_token(&mut self, token: AuthToken) -> Result<()> {
        token.save()?;
        self.token = Some(token);
        Ok(())
    }

    pub fn logout(&mut self) -> Result<()> {
        AuthToken::delete()?;
        self.token = None;
        Ok(())
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new().unwrap_or(Self { token: None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_manager_not_authenticated_by_default() {
        let manager = AuthManager { token: None };
        assert!(!manager.is_authenticated());
    }

    #[test]
    fn test_auth_token_expiry() {
        let expired_token = AuthToken {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: Utc::now() - chrono::Duration::hours(1),
            account_id: "test".to_string(),
        };
        assert!(expired_token.is_expired());

        let valid_token = AuthToken {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: Utc::now() + chrono::Duration::hours(1),
            account_id: "test".to_string(),
        };
        assert!(!valid_token.is_expired());
    }
}
