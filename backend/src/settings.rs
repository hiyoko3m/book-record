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

    #[serde(default = "default_refresh_key")]
    pub refresh_token_cookie_name: String,
}

fn default_refresh_key() -> String {
    "refresh_token".to_string()
}
