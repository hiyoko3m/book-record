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
    #[serde(default = "default_refresh_prefix")]
    pub refresh_prefix: String,
    #[serde(default = "default_refresh_exp")]
    pub refresh_exp: usize, // secs

    // Access token
    #[serde(default = "default_access_exp")]
    pub access_exp: usize, // secs
    #[serde(default = "default_access_iss")]
    pub access_iss: String,
    #[serde(default = "default_access_secret")]
    pub access_secret: String,

    #[serde(default = "default_refresh_key")]
    pub refresh_token_cookie_name: String,
}

fn default_login_session_prefix() -> String {
    "LS-".to_string()
}

fn default_login_session_exp() -> usize {
    900
}

fn default_sign_up_session_prefix() -> String {
    "SUS-".to_string()
}

fn default_sign_up_session_exp() -> usize {
    900
}

fn default_refresh_prefix() -> String {
    "REF-".to_string()
}

fn default_refresh_exp() -> usize {
    60 * 60 * 24 * 7
}

fn default_access_exp() -> usize {
    60 * 15
}

fn default_access_iss() -> String {
    "book-record".to_string()
}

fn default_access_secret() -> String {
    "SyBNLfDIYgjs6WF7I8YKMAQdFrDBeo1v8rTnM+PEHzA=".to_string()
}

fn default_refresh_key() -> String {
    "refresh_token".to_string()
}
