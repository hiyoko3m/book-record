use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

use super::super::entity::book::{BookEntity, BookEntityForCreation};
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

impl BookService {
    pub async fn list_books(&self) -> Vec<BookEntity> {
        self.book_repository.list_books().await
    }

    pub async fn get_book(&self, book_id: u32) -> Option<BookEntity> {
        self.book_repository.get_book(book_id).await
    }

    pub async fn create_book(&self, book: BookEntityForCreation) -> Result<u32, ()> {
        self.book_repository.create_book(book).await
    }

    /// Return: true indicates that the update operation was succeeded.
    pub async fn update_book(&self, book: BookEntity) -> bool {
        self.book_repository.update_book(book).await
    }

    /// Return: true indicates that the delete operation was succeeded.
    pub async fn delete_book(&self, book_id: u32) -> bool {
        self.book_repository.delete_book(book_id).await
    }
}
