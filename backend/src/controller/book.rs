use rocket::get;
use rocket::serde::json::{json, Value};

use super::models::Book;
use crate::domain::entity::book::BookEntity;
use crate::domain::service::book::BookService;

#[get("/books")]
pub async fn list_books(book_service: BookService) -> Value {
    let books: Vec<BookEntity> = book_service.list_books().await;
    json!({
        "books": books.into_iter().map(|book| book.into()).collect::<Vec<Book>>(),
    })
}
