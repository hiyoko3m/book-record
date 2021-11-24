use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use openidconnect::core::CoreClient;
use openidconnect::reqwest::async_http_client;
use openidconnect::{AuthorizationCode, Nonce, PkceCodeChallenge, PkceCodeVerifier, TokenResponse};
use redis::{AsyncCommands, Client as RedisClient};
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use super::session::LoginSessionStorage;
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

    async fn make_login_session(&self) -> Result<LoginSession, LoginError> {
        // OpenID Connectの仕様に沿ったコード生成
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        // openidconnect crateに沿った使い方ではないので、
        // この時点でString自体を取り出しておく
        let nonce = Nonce::new_random().secret().to_owned();

        // アプリ独自にセッションを用意し、
        // あとでclient sideから来るリクエストと
        // OpenID Connectのコードを対応付ける
        let session_id = Uuid::new_v4().to_string();
        let session_info = LoginSessionStorage::new(&nonce, pkce_verifier.secret());

        tracing::debug!(
            "in make_login_session: challenge: {}, verifier: {}",
            pkce_challenge.as_str(),
            pkce_verifier.secret()
        );

        tracing::info!("Login session start: {}", session_id);

        // Redisを使い、session_idと
        // nonceおよびpkce_verifierを関連付ける
        let mut con = self.redis_cli.get_async_connection().await.map_err(|err| {
            tracing::error!(
                "in make_login_session: error in making connection to Redis: {}",
                err
            );
            LoginError::Other
        })?;
        // 型を()に指定しないとコンパイルできない
        let _: () = con
            .set_ex(
                format!("{}{}", self.settings.login_session_prefix, session_id),
                serde_json::to_string(&session_info).unwrap(),
                self.settings.login_session_exp,
            )
            .await
            .map_err(|err| {
                tracing::error!(
                    "in make_login_session: error in storing session info ({}) to Redis: {}",
                    session_id,
                    err
                );
                LoginError::Other
            })?;

        Ok(LoginSession {
            session_id: session_id,
            nonce: nonce,
            code_challenge: pkce_challenge,
        })
    }

    async fn fetch_user_subject(
        &self,
        session_id: String,
        code: String,
    ) -> Result<String, LoginError> {
        // Redisからlogin session情報の取得
        let mut con = self.redis_cli.get_async_connection().await.map_err(|err| {
            tracing::error!(
                "in fetch_user_subject: error in making connection to Redis: {}",
                err
            );
            LoginError::Other
        })?;

        let info: String = con
            .get(format!(
                "{}{}",
                self.settings.login_session_prefix, session_id
            ))
            .await
            .map_err(|err| {
                tracing::info!(
                    "in fetch_user_subject: the login session with {} does not exist: {}",
                    session_id,
                    err
                );
                LoginError::InvalidCode
            })?;

        let info: LoginSessionStorage = serde_json::from_str(&info).map_err(|err| {
            tracing::error!("in fetch_user_subject: broken session info: {}", err);
            LoginError::Other
        })?;

        let nonce = Nonce::new(info.nonce);
        let pkce_verifier = PkceCodeVerifier::new(info.pkce_verifier);

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

    async fn issue_sign_up_code(&self, subject: String) -> Result<SignUpCode, SignUpError> {
        let code = Uuid::new_v4().to_string();

        // Redisにsign up session情報を保存
        let mut con = self.redis_cli.get_async_connection().await.map_err(|err| {
            tracing::error!(
                "in issue_sign_up_code: error in making connection to Redis: {}",
                err
            );
            SignUpError::Other
        })?;

        let _: () = con
            .set_ex(
                format!("{}{}", self.settings.sign_up_session_prefix, code),
                subject,
                self.settings.sign_up_session_exp,
            )
            .await
            .map_err(|err| {
                tracing::error!(
                    "in issue_sign_up_code: error in making sing up session: {}",
                    err
                );
                SignUpError::Other
            })?;

        Ok(SignUpCode::from(code))
    }

    async fn verify_sign_up_code(&self, code: SignUpCode) -> Result<String, SignUpError> {
        let mut con = self.redis_cli.get_async_connection().await.map_err(|err| {
            tracing::error!(
                "in verify_sign_up_code: error in making connection to Redis: {}",
                err
            );
            SignUpError::Other
        })?;

        con.get(format!(
            "{}{}",
            self.settings.sign_up_session_prefix,
            code.raw()
        ))
        .await
        .map_err(|err| {
            tracing::info!(
                "in verify_sign_up_code: invalid or expired sign up: {}",
                err
            );
            SignUpError::InvalidCode
        })
    }

    async fn issue_refresh_token(
        &self,
        userid: entity::PID,
    ) -> Result<RefreshToken, RefreshTokenError> {
        unimplemented!();
    }

    async fn verify_refresh_token(
        &self,
        token: RefreshTokenExtract,
    ) -> Result<entity::PID, RefreshTokenError> {
        unimplemented!();
    }
}
