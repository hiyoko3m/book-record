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

    // Redis's hash name
    #[serde(default = "default_login_session_hash_name")]
    pub login_session_hash_name: String,
    #[serde(default = "default_sign_up_session_hash_name")]
    pub sign_up_session_hash_name: String,

    #[serde(default = "default_refresh_key")]
    pub refresh_token_cookie_name: String,
}

fn default_login_session_hash_name() -> String {
    "login_session".to_string()
}

fn default_sign_up_session_hash_name() -> String {
    "sign_up_session".to_string()
}

fn default_refresh_key() -> String {
    "refresh_token".to_string()
}
