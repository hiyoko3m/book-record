use axum::{
    http::{Response, StatusCode},
    response::{IntoResponse, Json},
};
use chrono::{DateTime, Utc};
use openidconnect::PkceCodeChallenge;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::PID;

#[derive(Debug, Serialize)]
pub struct UserEntity {
    pub id: PID,
    pub subject: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UserEntityForCreation {
    pub username: String,
}

#[derive(Debug)]
pub enum UserError {
    Nonexistent,
    Duplicated,
    Other,
}

#[derive(Debug)]
pub struct LoginSession {
    pub session_id: String,
    pub nonce: String,
    pub code_challenge: PkceCodeChallenge,
}

pub enum LoginError {
    InvalidCode,
    Nonexistent(SignUpCode),
    IdTokenMissing,
    Other,
}

impl IntoResponse for LoginError {
    type Body = <(StatusCode, Json<Value>) as IntoResponse>::Body;
    type BodyError = <(StatusCode, Json<Value>) as IntoResponse>::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            LoginError::Nonexistent(code) => (StatusCode::FORBIDDEN, code.raw()),
            LoginError::InvalidCode | LoginError::IdTokenMissing => {
                (StatusCode::FORBIDDEN, String::new())
            }
            LoginError::Other => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct SignUpCode(String);

impl SignUpCode {
    pub fn raw(self) -> String {
        self.0
    }
}

impl From<String> for SignUpCode {
    fn from(s: String) -> Self {
        SignUpCode(s)
    }
}

pub enum SignUpError {
    InvalidCode,
    DuplicatedUser,
    Other,
}

impl IntoResponse for SignUpError {
    type Body = <StatusCode as IntoResponse>::Body;
    type BodyError = <StatusCode as IntoResponse>::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        match self {
            SignUpError::DuplicatedUser => StatusCode::BAD_REQUEST,
            SignUpError::InvalidCode => StatusCode::FORBIDDEN,
            SignUpError::Other => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

#[derive(Debug)]
pub struct RefreshToken {
    token: String,
    expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(token: String, expires_at: DateTime<Utc>) -> Self {
        Self { token, expires_at }
    }

    pub fn into_cookie_value(&self, cookie_name: &str, path: &str) -> String {
        format!(
            "{}={}; Expires={}; Path={}; HttpOnly",
            cookie_name,
            self.token,
            self.expires_at.to_rfc2822(),
            path,
        )
    }
}

#[derive(Debug)]
pub struct RefreshTokenExtract(pub String);

#[derive(Debug)]
pub struct AccessToken(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    iss: String,
    sub: PID,
    exp: usize,
}

impl AccessTokenClaims {
    pub fn new(iss: String, sub: PID, exp: usize) -> Self {
        Self { iss, sub, exp }
    }

    pub fn user_id(&self) -> PID {
        self.sub
    }
}

pub enum RefreshTokenError {
    InvalidRefreshToken,
    Other,
}

impl IntoResponse for RefreshTokenError {
    type Body = <StatusCode as IntoResponse>::Body;
    type BodyError = <StatusCode as IntoResponse>::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        match self {
            RefreshTokenError::InvalidRefreshToken => StatusCode::FORBIDDEN,
            RefreshTokenError::Other => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_refresh_token() {
        let refresh_token =
            RefreshToken::new("t0ken".to_string(), Utc.ymd(2000, 1, 1).and_hms(0, 1, 1));
        let expected =
            "refresh_token=t0ken; Expires=Sat, 01 Jan 2000 00:01:01 +0000; Path=/; HttpOnly";

        assert_eq!(
            refresh_token.into_cookie_value("refresh_token", "/"),
            expected
        );
    }
}
