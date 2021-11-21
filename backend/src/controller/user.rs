use axum::{
    extract::TypedHeader,
    http::StatusCode,
    response::{Headers, IntoResponse},
    routing::post,
    Json, Router,
};
use headers::Authorization;
use serde_json::Value;

use crate::controller::models::CredTokens;
use crate::domain::entity::user::IdTokenError;
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

async fn login(
    user_service: UserService,
    id_token: String,
) -> Result<CredTokens, (StatusCode, String)> {
    user_service
        .login(id_token)
        .await
        .map(|tokens| tokens.into())
        .map_err(|err| match err {
            IdTokenError::InvalidIdToken => (StatusCode::BAD_REQUEST, String::new()),
            IdTokenError::NonexistUser(token) => (StatusCode::UNAUTHORIZED, token),
        })
}

async fn sign_up(user_service: UserService) -> Json<Value> {
    unimplemented!();
}

async fn issue_access_token(user_service: UserService) -> Json<Value> {
    unimplemented!();
}
