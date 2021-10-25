use super::super::entity::book::BookEntity;

#[rocket::async_trait]
pub trait BookRepositoryInterface {
    async fn list_books(&self) -> Vec<BookEntity>;
}
