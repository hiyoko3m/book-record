pub mod book;
pub mod user;

use axum::{
    http::{Response, StatusCode},
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};

pub type PID = u32;

#[derive(Debug)]
pub enum AxumError {
    PgConnectionError,
    RedisConnectionError,
    OtherError(String),
}

impl IntoResponse for AxumError {
    type Body = <(StatusCode, Json<Value>) as IntoResponse>::Body;
    type BodyError = <(StatusCode, Json<Value>) as IntoResponse>::BodyError;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            AxumError::PgConnectionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Connecting Postgres error".to_string(),
            ),
            AxumError::RedisConnectionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Connecting Redis error".to_string(),
            ),
            AxumError::OtherError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("General error {}", message),
            ),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
