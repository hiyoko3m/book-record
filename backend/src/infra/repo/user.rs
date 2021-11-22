use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient};
use openidconnect::{AuthorizationCode, CsrfToken, Nonce, PkceCodeChallenge};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

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
    client: CoreClient,
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
        let Extension(client) = Extension::<CoreClient>::from_request(req)
            .await
            .map_err(error::internal_error)?;

        Ok(Self { pool, client })
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
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let nonce = Nonce::new_random();

        let session_id = Uuid::new_v4().to_string();

        // TODO
        // session_idからnonceとpkce_verifierが引けるように関連付け
        LoginSession {
            session_id: session_id,
            nonce: nonce.secret().to_owned(),
            code_challenge: pkce_challenge,
        }
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
