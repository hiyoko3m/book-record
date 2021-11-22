use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use chrono::{TimeZone, Utc};

use crate::domain::entity::{
    user::{
        AccessToken, LoginError, LoginSession, RefreshToken, RefreshTokenError,
        RefreshTokenExtract, SignUpCode, SignUpError, UserEntityForCreation,
    },
    PID,
};
use crate::domain::repository_interface::user::UserRepositoryInterface;
use crate::infrastructure::repository::user::UserRepository;

pub struct UserService {
    user_repository: UserRepository,
}

#[async_trait]
impl<B> FromRequest<B> for UserService
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_repository = UserRepository::from_request(req).await?;
        Ok(Self { user_repository })
    }
}

impl UserService {
    pub async fn make_login_session(&self) -> LoginSession {
        self.user_repository.make_login_session().await
    }

    async fn issue_tokens(&self, uid: PID) -> (RefreshToken, AccessToken) {
        let refresh_token = self.user_repository.issue_refresh_token(uid).await;
        let access_token = Self::issue_access_token(uid);
        (refresh_token, access_token)
    }

    pub async fn login(
        &self,
        session_id: String,
        code: String,
    ) -> Result<(RefreshToken, AccessToken), LoginError> {
        if let Some(sub) = self
            .user_repository
            .fetch_authed_user(session_id, code)
            .await
        {
            if let Some(uid) = self.user_repository.get_user_id_from_sub(&sub).await {
                Ok(self.issue_tokens(uid).await)
            } else {
                let code = self.user_repository.issue_sign_up_code(sub).await;
                Err(LoginError::NonexistUser(code))
            }
        } else {
            Err(LoginError::InvalidCode)
        }
    }

    pub async fn sign_up(
        &self,
        code: SignUpCode,
        user: UserEntityForCreation,
    ) -> Result<(RefreshToken, AccessToken), SignUpError> {
        if let Some(sub) = self.user_repository.verify_sign_up_code(code).await {
            let uid = self
                .user_repository
                .create_user(sub, user)
                .await
                .map_err(|err| {
                    println!("Create user error");
                    SignUpError::DuplicatedUser
                })?;
            Ok(self.issue_tokens(uid).await)
        } else {
            Err(SignUpError::InvalidCode)
        }
    }

    pub async fn refresh_tokens(
        &self,
        refresh_token: RefreshTokenExtract,
    ) -> Result<(RefreshToken, AccessToken), RefreshTokenError> {
        if let Some(uid) = self
            .user_repository
            .verify_refresh_token(refresh_token)
            .await
        {
            Ok(self.issue_tokens(uid).await)
        } else {
            Err(RefreshTokenError::InvalidRefreshToken)
        }
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
