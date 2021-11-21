use axum::{routing::post, Json, Router};
use serde_json::Value;

use crate::domain::service::user::UserService;

pub fn user_app() -> Router {
    Router::new()
        .route("/nonce", post(issue_nonce))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/token", post(issue_access_token))
}

async fn issue_nonce(user_service: UserService) -> Json<String> {
    unimplemented!();
}

async fn login(user_service: UserService) -> Json<Value> {
    unimplemented!();
}

async fn sign_up(user_service: UserService) -> Json<Value> {
    unimplemented!();
}

async fn issue_access_token(user_service: UserService) -> Json<Value> {
    unimplemented!();
}
