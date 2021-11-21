use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use sqlx::{postgres::PgPool, Row};

use crate::domain::entity::{
    user::{AccessToken, RefreshToken, UserEntity, UserEntityForCreation},
    PID as EPID,
};
use crate::domain::repository_interface::user::UserRepositoryInterface;
use crate::utils::error;

pub struct UserRepository {
    pool: PgPool,
}

#[async_trait]
impl<B> FromRequest<B> for UserRepository
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
impl UserRepositoryInterface for UserRepository {
    async fn get_user(&self, id: EPID) -> Option<UserEntity> {
        unimplemented!();
    }

    async fn get_user_from_sub(&self, sub: String) -> Option<UserEntity> {
        unimplemented!();
    }

    async fn create_user(&self, user: UserEntityForCreation) -> Result<u32, ()> {
        unimplemented!();
    }

    async fn issue_nonce(&self) -> String {
        unimplemented!();
    }

    async fn verity_nonce(&self, nonce: String) -> bool {
        unimplemented!();
    }

    async fn issue_refresh_token(&self, userid: EPID) -> RefreshToken {
        unimplemented!();
    }

    async fn verify_refresh_token(&self, token: RefreshToken) -> Option<EPID> {
        unimplemented!();
    }

    async fn issue_access_token(&self, userid: EPID) -> AccessToken {
        unimplemented!();
    }

    async fn verify_access_token(&self, token: AccessToken) -> Option<EPID> {
        unimplemented!();
    }
}
