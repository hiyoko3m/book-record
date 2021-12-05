pub mod book;
pub mod user;

use axum::{
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        Response, StatusCode,
    },
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};

pub type Pid = u32;

#[derive(Debug)]
pub enum AxumError {
    PgConnectionError,
    RedisConnectionError,
    MissingAccessToken,
    InvalidAccessToken,
    OtherError(String),
}

impl IntoResponse for AxumError {
    type Body = <(StatusCode, HeaderMap, Json<Value>) as IntoResponse>::Body;
    type BodyError = <(StatusCode, HeaderMap, Json<Value>) as IntoResponse>::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        let (status, headers, error_message) = match self {
            AxumError::PgConnectionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                "Connecting Postgres error".to_string(),
            ),
            AxumError::RedisConnectionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::new(),
                "Connecting Redis error".to_string(),
            ),
            AxumError::MissingAccessToken => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("www-authenticate"),
                    HeaderValue::from_static("Bearer"),
                );
                (StatusCode::UNAUTHORIZED, headers, String::new())
            }
            AxumError::InvalidAccessToken => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("www-authenticate"),
                    HeaderValue::from_static("Bearer error=\"invalid_token\""),
                );
                (StatusCode::UNAUTHORIZED, headers, String::new())
            }
            AxumError::OtherError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new(), message)
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, headers, body).into_response()
    }
}
