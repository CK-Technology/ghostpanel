use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfig {
    pub provider: OidcProvider,
    pub client_id: String,
    pub client_secret: Option<String>, // Not used in frontend
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub discovery_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OidcProvider {
    Azure {
        tenant_id: String,
    },
    Google,
    GitHub,
    Generic {
        auth_url: String,
        token_url: String,
        userinfo_url: String,
    },
}

impl OidcProvider {
    pub fn get_auth_url(&self, client_id: &str, redirect_uri: &str, state: &str) -> String {
        match self {
            OidcProvider::Azure { tenant_id } => {
                format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?\
                    client_id={}&\
                    response_type=code&\
                    redirect_uri={}&\
                    response_mode=query&\
                    scope=openid%20profile%20email&\
                    state={}",
                    tenant_id,
                    urlencoding::encode(client_id),
                    urlencoding::encode(redirect_uri),
                    urlencoding::encode(state)
                )
            }
            OidcProvider::Google => {
                format!(
                    "https://accounts.google.com/o/oauth2/v2/auth?\
                    client_id={}&\
                    response_type=code&\
                    redirect_uri={}&\
                    scope=openid%20profile%20email&\
                    state={}",
                    urlencoding::encode(client_id),
                    urlencoding::encode(redirect_uri),
                    urlencoding::encode(state)
                )
            }
            OidcProvider::GitHub => {
                format!(
                    "https://github.com/login/oauth/authorize?\
                    client_id={}&\
                    redirect_uri={}&\
                    scope=user:email&\
                    state={}",
                    urlencoding::encode(client_id),
                    urlencoding::encode(redirect_uri),
                    urlencoding::encode(state)
                )
            }
            OidcProvider::Generic { auth_url, .. } => {
                format!(
                    "{}?\
                    client_id={}&\
                    response_type=code&\
                    redirect_uri={}&\
                    scope=openid%20profile%20email&\
                    state={}",
                    auth_url,
                    urlencoding::encode(client_id),
                    urlencoding::encode(redirect_uri),
                    urlencoding::encode(state)
                )
            }
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            OidcProvider::Azure { .. } => "Azure AD",
            OidcProvider::Google => "Google",
            OidcProvider::GitHub => "GitHub",
            OidcProvider::Generic { .. } => "Custom OIDC",
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            OidcProvider::Azure { .. } => "🟦",
            OidcProvider::Google => "🟥",
            OidcProvider::GitHub => "⚫",
            OidcProvider::Generic { .. } => "🔑",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcTokenResponse {
    pub access_token: String,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
    pub preferred_username: Option<String>,

    // Provider-specific fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// OIDC authentication service for frontend
pub struct OidcService {
    configs: RwSignal<Vec<OidcConfig>>,
}

impl OidcService {
    pub fn new() -> Self {
        // In production, these would be loaded from backend
        let default_configs = vec![
            // These are examples - actual configs come from backend
        ];

        Self {
            configs: create_rw_signal(default_configs),
        }
    }

    pub fn get_providers(&self) -> Vec<OidcConfig> {
        self.configs.get()
    }

    pub fn start_auth_flow(&self, provider: &OidcConfig) -> String {
        let state = uuid::Uuid::new_v4().to_string();
        let redirect_uri = format!("{}/auth/callback", window().location().origin().unwrap());

        // Store state in session storage for validation
        if let Ok(storage) = window().session_storage() {
            if let Some(storage) = storage {
                let _ = storage.set_item("oidc_state", &state);
                let _ = storage.set_item("oidc_provider", &serde_json::to_string(&provider.provider).unwrap_or_default());
            }
        }

        provider.provider.get_auth_url(&provider.client_id, &redirect_uri, &state)
    }

    pub async fn handle_callback(&self, code: &str, state: &str) -> Result<UserInfo, String> {
        // Validate state
        if let Ok(storage) = window().session_storage() {
            if let Some(storage) = storage {
                if let Ok(Some(stored_state)) = storage.get_item("oidc_state") {
                    if stored_state != state {
                        return Err("Invalid state parameter".to_string());
                    }
                    let _ = storage.remove_item("oidc_state");
                } else {
                    return Err("No state found in session".to_string());
                }
            }
        }

        // Exchange code for tokens via backend
        let response = gloo_net::http::Request::post("/api/auth/oidc/callback")
            .json(&serde_json::json!({
                "code": code,
                "state": state
            }))
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !response.ok() {
            return Err(format!("Authentication failed: {}", response.status()));
        }

        let user_info: UserInfo = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(user_info)
    }
}

// Utility function to get window object
fn window() -> web_sys::Window {
    web_sys::window().expect("should have a window in this context")
}