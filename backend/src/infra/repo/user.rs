use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use openidconnect::core::CoreClient;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthorizationCode, Nonce, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use redis::{AsyncCommands, Client as RedisClient, Commands};
use serde_json::json;
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
use crate::settings::Settings;
use crate::utils::error;

pub struct UserRepositoryImpl {
    settings: Settings,
    pool: PgPool,
    redis_cli: RedisClient,
    client: CoreClient,
}

#[async_trait]
impl<B> FromRequest<B> for UserRepositoryImpl
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(settings) = Extension::<Settings>::from_request(req)
            .await
            .map_err(error::internal_error)?;
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(error::internal_error)?;
        let Extension(redis_cli) = Extension::<RedisClient>::from_request(req)
            .await
            .map_err(error::internal_error)?;
        let Extension(client) = Extension::<CoreClient>::from_request(req)
            .await
            .map_err(error::internal_error)?;

        Ok(Self {
            settings,
            pool,
            redis_cli,
            client,
        })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user(&self, id: entity::PID) -> Result<UserEntity, UserError> {
        unimplemented!();
    }

    async fn get_user_from_subject(&self, subject: &str) -> Result<UserEntity, UserError> {
        Err(UserError::Nonxistent)
    }

    async fn get_user_id_from_subject(&self, subject: &str) -> Result<entity::PID, UserError> {
        Err(UserError::Nonxistent)
    }

    async fn create_user(
        &self,
        subject: String,
        user: UserEntityForCreation,
    ) -> Result<entity::PID, UserError> {
        unimplemented!();
    }

    async fn make_login_session(&self) -> LoginSession {
        // OpenID Connectの仕様に沿ったコード生成
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        // openidconnect crateに沿った使い方ではないので、
        // この時点でString自体を取り出しておく
        let nonce = Nonce::new_random().secret().to_owned();

        // アプリ独自にセッションを用意し、
        // あとでclient sideから来るリクエストと
        // OpenID Connectのコードを対応付ける
        let session_id = Uuid::new_v4().to_string();

        tracing::debug!(
            "in make_login_session: challenge: {}, verifier: {}",
            pkce_challenge.as_str(),
            pkce_verifier.secret()
        );

        // Redisを使い、session_idと
        // nonceおよびpkce_verifierを関連付ける
        let mut con = self
            .redis_cli
            .get_async_connection()
            .await
            .expect("error in make connection to redis");
        // 型を()に指定しないとコンパイルできない
        let _: () = con
            .hset(
                self.settings.login_session_hash_name.to_owned(),
                session_id.to_owned(),
                json!({"nonce": nonce, "pkce_verifier": pkce_verifier.secret()}).to_string(),
            )
            .await
            .unwrap();

        LoginSession {
            session_id: session_id,
            nonce: nonce,
            code_challenge: pkce_challenge,
        }
    }

    async fn fetch_user_subject(
        &self,
        session_id: String,
        code: String,
    ) -> Result<String, LoginError> {
        // TODO
        // session_idからnonceとpkce_verifierを復元する
        let nonce = Nonce::new("rand0m".to_string());
        let pkce_verifier =
            PkceCodeVerifier::new("EB7lIQSNeq4PNjXLvRwQiT9HgWjdW22tM9g3h0WL3oM".to_string());

        // IdPでauthorization codeと引き換えてトークンをもらう
        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .map_err(|err| {
                tracing::info!(
                    "in fetch_authed_user: error in exchanging ID token, {:?}",
                    err
                );
                LoginError::InvalidCode
            })?;

        // IDトークンだけ取り出す
        let id_token = token_response.id_token().ok_or_else(|| {
            tracing::info!("in fetch_authed_user: ID token is missing");
            LoginError::IdTokenMissing
        })?;

        // IDトークンの検証とnonceの一致の確認
        // 検証はiss, audの一致と、署名について行われる（openidconnect v2.1.1のソースコードを確認）
        let claims = id_token
            .claims(&self.client.id_token_verifier(), &nonce)
            .map_err(|err| {
                tracing::info!(
                    "in fetch_authed_user: ID token verification failure: {:?}",
                    err
                );
                LoginError::InvalidCode
            })?;

        Ok(claims.subject().as_str().to_string())
    }

    async fn issue_sign_up_code(&self, sub: String) -> SignUpCode {
        SignUpCode::from("aiueo".to_string())
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
