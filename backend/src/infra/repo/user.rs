use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use openidconnect::core::CoreClient;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthorizationCode, Nonce, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use crate::domain::entity::{
    self,
    user::{
        LoginError, LoginSession, RefreshToken, RefreshTokenError, RefreshTokenExtract, SignUpCode,
        SignUpError, UserEntity, UserEntityForCreation, UserError,
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
    async fn get_user(&self, id: entity::PID) -> Result<UserEntity, UserError> {
        unimplemented!();
    }

    async fn get_user_from_sub(&self, sub: &str) -> Result<UserEntity, UserError> {
        unimplemented!();
    }

    async fn get_user_id_from_sub(&self, sub: &str) -> Result<entity::PID, UserError> {
        unimplemented!();
    }

    async fn create_user(
        &self,
        sub: String,
        user: UserEntityForCreation,
    ) -> Result<entity::PID, UserError> {
        unimplemented!();
    }

    async fn make_login_session(&self) -> LoginSession {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let nonce = Nonce::new_random();

        let session_id = Uuid::new_v4().to_string();

        tracing::info!(
            "challenge: {}, verifier: {}",
            pkce_challenge.as_str(),
            pkce_verifier.secret()
        );

        // TODO
        // session_idからnonceとpkce_verifierが引けるように関連付け
        LoginSession {
            session_id: session_id,
            nonce: nonce.secret().to_owned(),
            code_challenge: pkce_challenge,
        }
    }

    async fn fetch_authed_user(
        &self,
        session_id: String,
        code: String,
    ) -> Result<String, LoginError> {
        // TODO
        // session_idからnonceとpkce_verifierを復元する

        let pkce_verifier =
            PkceCodeVerifier::new("EB7lIQSNeq4PNjXLvRwQiT9HgWjdW22tM9g3h0WL3oM".to_string());

        // IdPでauthorization codeと引き換えてトークンをもらう
        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .expect("Error on fetch id token");

        // IDトークンだけ取り出す
        let id_token = token_response
            .id_token()
            .expect("Error on convert token to id_token");

        tracing::info!("{:?}", id_token);

        Err(LoginError::InvalidCode)
    }

    async fn issue_sign_up_code(&self, sub: String) -> SignUpCode {
        unimplemented!();
    }

    async fn verify_sign_up_code(&self, code: SignUpCode) -> Result<String, SignUpError> {
        unimplemented!();
    }

    async fn issue_refresh_token(&self, userid: entity::PID) -> RefreshToken {
        unimplemented!();
    }

    async fn verify_refresh_token(
        &self,
        token: RefreshTokenExtract,
    ) -> Result<entity::PID, RefreshTokenError> {
        unimplemented!();
    }
}
