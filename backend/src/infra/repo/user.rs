use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use sqlx::{postgres::PgPool, Row};

use crate::domain::entity::{
    self,
    user::{
        LoginSession, RefreshToken, RefreshTokenExtract, SignUpCode, UserEntity,
        UserEntityForCreation,
    },
};
use crate::domain::repo_if::user::UserRepository;
use crate::utils::error;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
impl<B> FromRequest<B> for UserRepositoryImpl
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(error::internal_error)?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user(&self, id: entity::PID) -> Option<UserEntity> {
        unimplemented!();
    }

    async fn get_user_from_sub(&self, sub: &str) -> Option<UserEntity> {
        unimplemented!();
    }

    async fn get_user_id_from_sub(&self, sub: &str) -> Option<entity::PID> {
        unimplemented!();
    }

    async fn create_user(
        &self,
        sub: String,
        user: UserEntityForCreation,
    ) -> Result<entity::PID, ()> {
        unimplemented!();
    }

    async fn make_login_session(&self) -> LoginSession {
        unimplemented!();
    }

    async fn fetch_authed_user(&self, session_id: String, code: String) -> Option<String> {
        unimplemented!();
    }

    async fn issue_sign_up_code(&self, sub: String) -> SignUpCode {
        unimplemented!();
    }

    async fn verify_sign_up_code(&self, code: SignUpCode) -> Option<String> {
        unimplemented!();
    }

    async fn issue_refresh_token(&self, userid: entity::PID) -> RefreshToken {
        unimplemented!();
    }

    async fn verify_refresh_token(&self, token: RefreshTokenExtract) -> Option<entity::PID> {
        unimplemented!();
    }
}
