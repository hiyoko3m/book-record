use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

use super::super::entity::book::BookEntity;
use super::super::repository_interface::book::BookRepositoryInterface;
use crate::infrastructure::repository::book::BookRepository;

pub struct BookService {
    book_repository: BookRepository,
}

#[async_trait]
impl<B> FromRequest<B> for BookService
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let book_repository = BookRepository::from_request(req).await?;
        Ok(Self { book_repository })
    }
}

#[async_trait]
pub trait Listable {
    async fn list_books(&self) -> Vec<BookEntity>;
}

#[async_trait]
impl Listable for BookService {
    async fn list_books(&self) -> Vec<BookEntity> {
        self.book_repository.list_books().await
    }
}
