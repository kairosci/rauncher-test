use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::auth::AuthToken;
use crate::{Error, Result};

// Epic Games Store API endpoints
#[allow(dead_code)]
const EGS_API_BASE: &str = "https://api.epicgames.dev";
const OAUTH_TOKEN_URL: &str =
    "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

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
    /// This is a simplified implementation - real implementation would need OAuth device flow
    pub async fn authenticate(&self, _auth_code: &str) -> Result<AuthToken> {
        // In a real implementation, this would:
        // 1. Request a device code
        // 2. Show the user a URL and code to enter
        // 3. Poll for authentication completion
        // 4. Exchange the code for tokens

        // For now, return an error indicating this needs to be implemented
        Err(Error::Auth(
            "Authentication not yet implemented. This requires Epic Games OAuth setup.".to_string(),
        ))
    }

    /// Refresh an expired access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken> {
        let response = self
            .client
            .post(OAUTH_TOKEN_URL)
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Auth(format!(
                "Failed to refresh token: {}",
                response.status()
            )));
        }

        let oauth_response: OAuthTokenResponse = response.json().await?;

        Ok(AuthToken {
            access_token: oauth_response.access_token,
            refresh_token: oauth_response.refresh_token,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(oauth_response.expires_in),
            account_id: oauth_response.account_id,
        })
    }

    /// Get the user's game library
    pub async fn get_games(&self, _token: &AuthToken) -> Result<Vec<Game>> {
        // This would make an authenticated request to Epic's API to get the user's library
        // For now, return an empty list as we don't have valid tokens
        Ok(vec![])
    }

    /// Get game manifest for download
    pub async fn get_game_manifest(&self, _token: &AuthToken, _app_name: &str) -> Result<String> {
        // This would fetch the game manifest which contains download information
        Err(Error::Api(
            "Manifest download not yet implemented".to_string(),
        ))
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
}
