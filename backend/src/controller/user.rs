use axum::{
    extract::TypedHeader,
    http::StatusCode,
    response::{Headers, IntoResponse},
    routing::post,
    Json, Router,
};
use chrono::{TimeZone, Utc};
use headers::Authorization;
use serde_json::Value;

use crate::domain::entity::user::{AccessToken, IdTokenError, RefreshToken};
use crate::domain::service::user::UserService;

pub fn user_app() -> Router {
    Router::new()
        .route("/nonce", post(issue_nonce))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/token", post(issue_access_token))
}

async fn issue_nonce(user_service: UserService) -> Json<String> {
    Json(user_service.issue_nonce().await)
}

fn response_from_tokens(
    refresh_token: RefreshToken,
    access_token: AccessToken,
) -> impl IntoResponse {
    (
        Headers(vec![("Set-Cookie", refresh_token.into_cookie_value())]),
        access_token.0,
    )
}

async fn login(
    user_service: UserService,
    id_token: String,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    user_service
        .login(id_token)
        .await
        .map(|ts| response_from_tokens(ts.0, ts.1))
        .map_err(|err| match err {
            IdTokenError::NonexistUser(token) => (StatusCode::BAD_REQUEST, token),
            IdTokenError::InvalidIdToken => (StatusCode::UNAUTHORIZED, String::new()),
        })
}

async fn sign_up(user_service: UserService) -> Json<Value> {
    unimplemented!();
}

async fn issue_access_token(user_service: UserService) -> Json<Value> {
    unimplemented!();
}
