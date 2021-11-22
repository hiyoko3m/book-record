use axum::{
    extract::TypedHeader,
    http::StatusCode,
    response::{Headers, IntoResponse},
    routing::post,
    Json, Router,
};
use headers::Cookie;

use crate::controller::models::SignUpExtract;
use crate::domain::entity::user::{
    AccessToken, IssueAccessTokenError, LoginError, RefreshToken, RefreshTokenExtract, SignUpError,
};
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
            LoginError::NonexistUser(token) => (StatusCode::BAD_REQUEST, token.raw()),
            LoginError::InvalidIdToken => (StatusCode::FORBIDDEN, String::new()),
        })
}

async fn sign_up(
    user_service: UserService,
    Json(payload): Json<SignUpExtract>,
) -> Result<impl IntoResponse, StatusCode> {
    user_service
        .sign_up(payload.token, payload.user)
        .await
        .map(|ts| response_from_tokens(ts.0, ts.1))
        .map_err(|err| match err {
            SignUpError::DuplicatedUser => StatusCode::BAD_REQUEST,
            SignUpError::InvalidSignUpToken => StatusCode::FORBIDDEN,
        })
}

async fn issue_access_token(
    user_service: UserService,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<impl IntoResponse, StatusCode> {
    let refresh_token_value = cookie.get("refresh_token").ok_or(StatusCode::FORBIDDEN)?;

    user_service
        .issue_access_token(RefreshTokenExtract(refresh_token_value.to_string()))
        .await
        .map(|ts| response_from_tokens(ts.0, ts.1))
        .map_err(|err| match err {
            IssueAccessTokenError::InvalidRefreshToken => StatusCode::FORBIDDEN,
        })
}
