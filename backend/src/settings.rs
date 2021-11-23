use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: String,

    // OpenID Connectのprovider
    // 最後にslashを入れてはいけない
    pub id_provider_url: String,
    pub id_provider_client_id: String,
    pub id_provider_client_secret: String,
    pub id_provider_redirect_url: String,

    // Redis
    #[serde(default = "default_login_session_prefix")]
    pub login_session_prefix: String,
    #[serde(default = "default_login_session_exp")]
    pub login_session_exp: usize, // secs
    #[serde(default = "default_sign_up_session_prefix")]
    pub sign_up_session_prefix: String,
    #[serde(default = "default_sign_up_session_exp")]
    pub sign_up_session_exp: usize, // secs

    #[serde(default = "default_refresh_key")]
    pub refresh_token_cookie_name: String,
}

fn default_login_session_prefix() -> String {
    "LS-".to_string()
}

fn default_login_session_exp() -> usize {
    15
}

fn default_sign_up_session_prefix() -> String {
    "SUS-".to_string()
}

fn default_sign_up_session_exp() -> usize {
    15
}

fn default_refresh_key() -> String {
    "refresh_token".to_string()
}
