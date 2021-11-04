use super::super::entity::book::BookEntity;
use axum::async_trait;

#[async_trait]
pub trait BookRepositoryInterface {
    async fn list_books(&self) -> Vec<BookEntity>;
}
