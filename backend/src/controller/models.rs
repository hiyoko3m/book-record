use serde::Deserialize;

use crate::domain::entity::user::{SignUpToken, UserEntityForCreation};

#[derive(Debug, Deserialize)]
pub struct SignUpExtract {
    pub token: SignUpToken,
    pub user: UserEntityForCreation,
}
