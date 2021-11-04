use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

use diesel::RunQueryDsl;

use super::connection::DatabaseConnection;
use crate::domain::entity::book::BookEntity;
use crate::domain::repository_interface::book::BookRepositoryInterface;

pub struct BookRepository {
    conn: DatabaseConnection,
}

#[async_trait]
impl<B> FromRequest<B> for BookRepository
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let conn = DatabaseConnection::from_request(req).await?;
        Ok(Self { conn })
    }
}

#[async_trait]
impl BookRepositoryInterface for BookRepository {
    async fn list_books(&self) -> Vec<BookEntity> {
        use super::schema::books::dsl::*;
        books.load::<BookEntity>(&*self.conn.0).unwrap_or(vec![])
    }
}
