use serde::Deserialize;

use crate::domain::entity::user::{SignUpCode, UserEntityForCreation};

#[derive(Debug, Deserialize)]
pub struct LoginExtract {
    pub session_id: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct SignUpExtract {
    pub code: SignUpCode,
    pub user: UserEntityForCreation,
}
