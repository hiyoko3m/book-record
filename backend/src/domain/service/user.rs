use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use chrono::{TimeZone, Utc};

use crate::domain::entity::{
    user::{
        AccessToken, LoginError, RefreshToken, SignUpError, SignUpToken, UserEntityForCreation,
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
    pub async fn issue_nonce(&self) -> String {
        self.user_repository.issue_nonce().await
    }

    pub async fn login(&self, id_token: String) -> Result<(RefreshToken, AccessToken), LoginError> {
        unimplemented!();
    }

    pub async fn sign_up(
        &self,
        sign_up_token: SignUpToken,
        user: UserEntityForCreation,
    ) -> Result<(RefreshToken, AccessToken), SignUpError> {
        // TODO
        Ok((
            RefreshToken::new(sign_up_token.raw(), Utc.ymd(2021, 1, 1).and_hms(0, 1, 1)),
            AccessToken(user.username),
        ))
    }
}
