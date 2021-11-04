use axum::{handler::get, response::Json, routing::BoxRoute, Router};
use serde_json::{json, Value};

use super::models::Book;
use crate::domain::entity::book::BookEntity;
use crate::domain::service::book::BookService;
use crate::domain::service::book::Listable;

pub fn book_app() -> Router<BoxRoute> {
    Router::new().route("/books", get(list_books)).boxed()
}

async fn list_books(book_service: BookService) -> Json<Value> {
    let books: Vec<BookEntity> = book_service.list_books().await;
    Json(json!({
        "books": Vec::<Vec<Book>>::new(),
        //"books": books.into_iter().map(|book| book.into()).collect::<Vec<Book>>(),
    }))
}
