use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::auth::AuthToken;
use crate::{Error, Result};

// Epic Games Store API endpoints
const OAUTH_TOKEN_URL: &str =
    "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
const DEVICE_AUTH_URL: &str =
    "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/deviceAuthorization";
const LIBRARY_API_URL: &str =
    "https://library-service.live.use1a.on.epicgames.com/library/api/public";
const LAUNCHER_API_URL: &str =
    "https://launcher-public-service-prod.ol.epicgames.com/launcher/api/public";

// Epic Games launcher client credentials (publicly available)
const CLIENT_ID: &str = "34a02cf8f4414e29b15921876da36f9a";
const CLIENT_SECRET: &str = "daafbccc737745039dffe53d94fc76cf";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub app_name: String,
    pub app_title: String,
    pub app_version: String,
    pub install_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceAuthResponse {
    verification_uri_complete: String,
    user_code: String,
    device_code: String,
    expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryResponse {
    records: Vec<LibraryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LibraryItem {
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "namespace")]
    _namespace: String,
    #[serde(rename = "catalogItemId")]
    catalog_item_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetResponse {
    id: String,
    #[serde(rename = "appName")]
    app_name: String,
    label_name: String,
    metadata: AssetMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssetMetadata {
    #[serde(rename = "applicationId")]
    application_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct CatalogItem {
    id: String,
    title: String,
    #[serde(rename = "currentVersion")]
    current_version: Option<String>,
}

pub struct EpicClient {
    client: Client,
}

impl EpicClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("r-games-launcher/0.1.0")
            .build()?;

        Ok(Self { client })
    }

    /// Authenticate with Epic Games using device code flow
    pub async fn authenticate(&self) -> Result<(String, String, AuthToken)> {
        // Step 1: Request device authorization
        log::info!("Requesting device authorization from Epic Games");
        
        let device_auth_response = self
            .client
            .post(DEVICE_AUTH_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
            .send()
            .await?;

        if !device_auth_response.status().is_success() {
            let status = device_auth_response.status();
            let error_text = device_auth_response.text().await.unwrap_or_default();
            return Err(Error::Auth(format!(
                "Failed to request device authorization: {} - {}",
                status, error_text
            )));
        }

        let device_auth: DeviceAuthResponse = device_auth_response.json().await?;
        
        log::debug!(
            "Device code received. Verification URL: {}",
            device_auth.verification_uri_complete
        );

        // Step 2: Poll for token
        let device_code = device_auth.device_code.clone();
        let user_code = device_auth.user_code.clone();
        let verification_url = device_auth.verification_uri_complete.clone();
        
        // Poll every 5 seconds for up to 10 minutes
        let max_attempts = 120; // 10 minutes
        let poll_interval = Duration::from_secs(5);
        
        for attempt in 0..max_attempts {
            if attempt > 0 {
                tokio::time::sleep(poll_interval).await;
            }
            
            log::debug!("Polling for token (attempt {}/{})", attempt + 1, max_attempts);
            
            let response = self
                .client
                .post(OAUTH_TOKEN_URL)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
                .form(&[
                    ("grant_type", "device_code"),
                    ("device_code", &device_code),
                ])
                .send()
                .await?;

            if response.status().is_success() {
                let oauth_response: OAuthTokenResponse = response.json().await?;
                
                log::info!("Successfully authenticated with Epic Games");
                
                let token = AuthToken {
                    access_token: oauth_response.access_token,
                    refresh_token: oauth_response.refresh_token,
                    expires_at: chrono::Utc::now()
                        + chrono::Duration::seconds(oauth_response.expires_in),
                    account_id: oauth_response.account_id,
                };
                
                return Ok((user_code, verification_url, token));
            }
            
            // Check if we got an error that means we should continue polling
            let status = response.status();
            if status == 400 {
                // This is expected while waiting for user to authenticate
                log::debug!("Still waiting for user authentication...");
                continue;
            }
            
            // Any other error should be reported
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Auth(format!(
                "Authentication failed: {} - {}",
                status, error_text
            )));
        }
        
        Err(Error::Auth(
            "Authentication timed out. Please try again.".to_string(),
        ))
    }

    /// Refresh an expired access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken> {
        log::info!("Refreshing access token");
        
        let response = self
            .client
            .post(OAUTH_TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Auth(format!(
                "Failed to refresh token: {} - {}",
                status, error_text
            )));
        }

        let oauth_response: OAuthTokenResponse = response.json().await?;
        
        log::info!("Successfully refreshed access token");

        Ok(AuthToken {
            access_token: oauth_response.access_token,
            refresh_token: oauth_response.refresh_token,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(oauth_response.expires_in),
            account_id: oauth_response.account_id,
        })
    }

    /// Get the user's game library
    pub async fn get_games(&self, token: &AuthToken) -> Result<Vec<Game>> {
        log::info!("Fetching game library from Epic Games");
        
        let library_url = format!(
            "{}/users/{}/items",
            LIBRARY_API_URL, token.account_id
        );
        
        let response = self
            .client
            .get(&library_url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Api(format!(
                "Failed to fetch library: {} - {}",
                status, error_text
            )));
        }

        let library_response: LibraryResponse = response.json().await?;
        
        log::debug!("Found {} items in library", library_response.records.len());
        
        // Convert library items to games
        // Note: We need to fetch additional details for each game
        let mut games = Vec::new();
        
        for item in library_response.records {
            // For now, we'll create basic game entries
            // In a full implementation, we'd fetch catalog details for each
            games.push(Game {
                app_name: item.app_name.clone(),
                app_title: item.app_name.clone(), // Will be replaced with catalog lookup
                app_version: "unknown".to_string(), // Will be replaced with catalog lookup
                install_path: None,
            });
        }
        
        log::info!("Successfully fetched {} games from library", games.len());
        
        Ok(games)
    }

    /// Get game manifest URL for download
    pub async fn get_game_manifest(&self, token: &AuthToken, app_name: &str) -> Result<String> {
        log::info!("Fetching manifest for game: {}", app_name);
        
        // Get asset information from launcher API
        let asset_url = format!(
            "{}/assets/Windows?label=Live",
            LAUNCHER_API_URL
        );
        
        let response = self
            .client
            .get(&asset_url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Api(format!(
                "Failed to fetch assets: {} - {}",
                status, error_text
            )));
        }

        let assets: Vec<AssetResponse> = response.json().await?;
        
        // Find the asset for the requested app
        let asset = assets
            .iter()
            .find(|a| a.app_name.eq_ignore_ascii_case(app_name))
            .ok_or_else(|| Error::GameNotFound(app_name.to_string()))?;
        
        log::info!("Found asset for {}: {}", app_name, asset.id);
        
        // Return the asset ID which would be used to construct manifest URL
        // In a real implementation, we would fetch the actual manifest from CDN
        Ok(asset.id.clone())
    }
}

impl Default for EpicClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epic_client_creation() {
        let client = EpicClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_game_serialization() {
        let game = Game {
            app_name: "test_app".to_string(),
            app_title: "Test Game".to_string(),
            app_version: "1.0.0".to_string(),
            install_path: None,
        };
        let serialized = serde_json::to_string(&game).unwrap();
        let deserialized: Game = serde_json::from_str(&serialized).unwrap();
        assert_eq!(game.app_name, deserialized.app_name);
    }

    #[test]
    fn test_oauth_token_response_deserialization() {
        let json = r#"{
            "access_token": "test_access",
            "refresh_token": "test_refresh",
            "expires_in": 3600,
            "account_id": "test_account"
        }"#;
        let response: OAuthTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.access_token, "test_access");
        assert_eq!(response.expires_in, 3600);
    }

    #[test]
    fn test_library_response_deserialization() {
        let json = r#"{
            "records": [
                {
                    "appName": "Fortnite",
                    "namespace": "fn",
                    "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5"
                }
            ]
        }"#;
        let response: LibraryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.records.len(), 1);
        assert_eq!(response.records[0].app_name, "Fortnite");
    }
}
