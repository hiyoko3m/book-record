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
    token: String,
}

#[derive(Debug)]
pub struct AccessToken {
    token: String,
}
