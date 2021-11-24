use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

use crate::domain::entity::{
    user::{
        AccessToken, LoginError, LoginSession, RefreshToken, RefreshTokenError,
        RefreshTokenExtract, SignUpCode, SignUpError, UserEntityForCreation,
    },
    PID,
};
use crate::domain::repo_if::user::UserRepository;
use crate::infra::repo::user::UserRepositoryImpl;

pub struct UserService {
    user_repository: UserRepositoryImpl,
}

#[async_trait]
impl<B> FromRequest<B> for UserService
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_repository = UserRepositoryImpl::from_request(req).await?;
        Ok(Self { user_repository })
    }
}

impl UserService {
    pub async fn make_login_session(&self) -> Result<LoginSession, LoginError> {
        self.user_repository.make_login_session().await
    }

    async fn issue_tokens(
        &self,
        uid: PID,
    ) -> Result<(RefreshToken, AccessToken), RefreshTokenError> {
        let refresh_token = self.user_repository.issue_refresh_token(uid).await?;
        let access_token = Self::issue_access_token(uid);
        Ok((refresh_token, access_token))
    }

    pub async fn login(
        &self,
        session_id: String,
        code: String,
    ) -> Result<(RefreshToken, AccessToken), LoginError> {
        // TODO
        // あとでモック化
        // それまではダミーのsubjectを使う
        let subject = self
            .user_repository
            .fetch_user_subject(session_id, code)
            .await?;
        //let subject = "dummy".to_string();
        tracing::info!("sub: {}", subject);

        if let Ok(uid) = self
            .user_repository
            .get_user_id_from_subject(&subject)
            .await
        {
            self.issue_tokens(uid).await.map_err(|_| LoginError::Other)
        } else {
            let code = self
                .user_repository
                .issue_sign_up_code(subject)
                .await
                .map_err(|_| LoginError::Other)?;
            Err(LoginError::Nonexistent(code))
        }
    }

    pub async fn sign_up(
        &self,
        code: SignUpCode,
        user: UserEntityForCreation,
    ) -> Result<(RefreshToken, AccessToken), SignUpError> {
        let subject = self.user_repository.verify_sign_up_code(code).await?;

        let uid = self
            .user_repository
            .create_user(subject, user)
            .await
            .map_err(|_| SignUpError::DuplicatedUser)?;
        self.issue_tokens(uid).await.map_err(|_| SignUpError::Other)
    }

    pub async fn refresh_tokens(
        &self,
        refresh_token: RefreshTokenExtract,
    ) -> Result<(RefreshToken, AccessToken), RefreshTokenError> {
        let uid = self
            .user_repository
            .verify_refresh_token(refresh_token)
            .await?;

        self.issue_tokens(uid).await
    }

    fn issue_access_token(uid: PID) -> AccessToken {
        unimplemented!();
    }

    /// Access tokenを検証する。
    /// Tokenが正しければ、token内にあるuser id情報を抽出して返す。
    fn verify_access_token(token: AccessToken) -> Option<PID> {
        unimplemented!();
    }
}
