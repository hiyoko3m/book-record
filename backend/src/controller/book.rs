use axum::{
    extract::{self, Path},
    handler::{get, post},
    http::StatusCode,
    response,
    routing::BoxRoute,
    Router,
};
use serde_json::{json, Value};

use super::models::BookExtract;
use crate::domain::entity::book::BookEntity;
use crate::domain::service::book::BookService;

pub fn book_app() -> Router<BoxRoute> {
    Router::new()
        .route("/books", get(list_books))
        .route("/book", post(create_book))
        .route(
            "/book/:id",
            get(get_book).post(update_book).delete(delete_book),
        )
        .boxed()
}

async fn list_books(book_service: BookService) -> response::Json<Value> {
    let books: Vec<BookEntity> = book_service.list_books().await;
    response::Json(json!({
        "books": books,
    }))
}

async fn get_book(
    book_service: BookService,
    Path(book_id): Path<u32>,
) -> Result<response::Json<Value>, StatusCode> {
    let book: Option<BookEntity> = book_service.get_book(book_id).await;
    book.map(|book| {
        response::Json(json!({
            "book": book,
        }))
    })
    .ok_or(StatusCode::NOT_FOUND)
}

async fn create_book(
    book_service: BookService,
    extract::Json(payload): extract::Json<BookExtract>,
) -> Result<response::Json<Value>, StatusCode> {
    let book_id = book_service.create_book(payload.into()).await;
    book_id
        .map(|book_id| {
            response::Json(json!({
                "book_id": book_id,
            }))
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_book(
    book_service: BookService,
    Path(book_id): Path<u32>,
    extract::Json(payload): extract::Json<BookExtract>,
) -> StatusCode {
    let result = book_service.update_book((book_id, payload).into()).await;
    if result {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn delete_book(book_service: BookService, Path(book_id): Path<u32>) -> StatusCode {
    let result = book_service.delete_book(book_id).await;
    if result {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
