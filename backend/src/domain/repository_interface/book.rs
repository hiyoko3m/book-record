use axum::async_trait;

use super::super::entity::book::{BookEntity, BookEntityForCreation};

#[async_trait]
pub trait BookRepositoryInterface {
    async fn list_books(&self) -> Vec<BookEntity>;

    async fn get_book(&self, book_id: u32) -> Option<BookEntity>;

    async fn create_book(&self, book: BookEntityForCreation) -> Result<u32, ()>;

    /// Return: true indicates that the update operation was succeeded.
    async fn update_book(&self, book: BookEntity) -> bool;

    /// Return: true indicates that the delete operation was succeeded.
    async fn delete_book(&self, book_id: u32) -> bool;
}
