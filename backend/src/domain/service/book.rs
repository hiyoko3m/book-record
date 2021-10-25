use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome, Request};

use super::super::entity::book::BookEntity;
use super::super::repository_interface::book::BookRepositoryInterface;
use crate::infrastructure::repository::book::BookRepository;

pub struct BookService {
    book_repository: BookRepository,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BookService {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let book_repository = try_outcome!(req.guard::<BookRepository>().await);
        Outcome::Success(Self { book_repository })
    }
}

impl BookService {
    pub async fn list_books(&self) -> Vec<BookEntity> {
        self.book_repository.list_books().await
    }
}
