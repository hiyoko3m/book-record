use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

pub struct UserService {}

#[async_trait]
impl<B> FromRequest<B> for UserService
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        unimplemented!();
    }
}
