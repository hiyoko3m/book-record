use serde::Deserialize;

use crate::domain::entity::user::{SignUpCode, UserEntityForCreation};

pub struct Settings {
    pub refresh_token_cookie_name: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            refresh_token_cookie_name: "refresh_token".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SignUpExtract {
    pub code: SignUpCode,
    pub user: UserEntityForCreation,
}
