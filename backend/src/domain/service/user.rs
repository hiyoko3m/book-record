use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

use crate::domain::entity::{
    user::{AccessToken, IdTokenError, RefreshToken},
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

    pub async fn login(
        &self,
        id_token: String,
    ) -> Result<(RefreshToken, AccessToken), IdTokenError> {
        unimplemented!();
    }
}
