use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use super::PID;

#[derive(Debug, Serialize)]
pub struct UserEntity {
    pub id: PID,
    pub sub: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UserEntityForCreation {
    pub username: String,
}

pub enum LoginError {
    InvalidIdToken,
    NonexistUser(SignUpToken),
}

#[derive(Debug, Deserialize)]
pub struct SignUpToken(String);

impl SignUpToken {
    pub fn raw(self) -> String {
        self.0
    }
}

impl SignUpToken {
    pub fn new() -> Self {
        Self(String::new())
    }
}

pub enum SignUpError {
    InvalidSignUpToken,
    DuplicatedUser,
}

#[derive(Debug)]
pub struct RefreshToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(token: String, expires_at: DateTime<Utc>) -> Self {
        Self { token, expires_at }
    }

    pub fn into_cookie_value(&self, cookie_name: &str) -> String {
        format!(
            "{}={}; Expires={}; Path=/; HttpOnly",
            cookie_name,
            self.token,
            self.expires_at.to_rfc2822()
        )
    }
}

#[derive(Debug)]
pub struct RefreshTokenExtract(pub String);

#[derive(Debug)]
pub struct AccessToken(pub String);

pub enum IssueAccessTokenError {
    InvalidRefreshToken,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_token() {
        let refresh_token =
            RefreshToken::new("t0ken".to_string(), Utc.ymd(2000, 1, 1).and_hms(0, 1, 1));
        let expected =
            "refresh_token=t0ken; Expires=Sat, 01 Jan 2000 00:01:01 +0000; Path=/; HttpOnly";

        assert_eq!(refresh_token.into_cookie_value(), expected);
    }
}
