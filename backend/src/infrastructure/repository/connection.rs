use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
};
use bb8::{Pool, PooledConnection};
use bb8_diesel::DieselConnectionManager;
use diesel::pg::PgConnection;

use crate::utils::error;

type ConnectionPool = Pool<DieselConnectionManager<PgConnection>>;

pub struct DatabaseConnection(pub PooledConnection<'static, DieselConnectionManager<PgConnection>>);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<ConnectionPool>::from_request(req)
            .await
            .map_err(error::internal_error)?;

        let conn = pool.get_owned().await.map_err(error::internal_error)?;

        Ok(Self(conn))
    }
}
