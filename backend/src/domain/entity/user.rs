use chrono::{DateTime, Utc};
use serde::Serialize;

use super::PID;

#[derive(Debug, Serialize)]
pub struct UserEntity {
    pub id: PID,
    pub sub: String,
    pub username: String,
}

#[derive(Debug)]
pub struct UserEntityForCreation {
    pub sub: String,
    pub username: String,
}

#[derive(Debug)]
pub struct RefreshToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct AccessToken(pub String);

pub enum IdTokenError {
    InvalidIdToken,
    NonexistUser(String), // signup token
}
