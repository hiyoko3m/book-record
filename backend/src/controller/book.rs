use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::domain::entity::book::{BookEntity, BookEntityForCreation};
use crate::domain::service::book::BookService;
use crate::domain::service::user::UserId;

pub fn book_app() -> Router {
    Router::new()
        .route("/books", get(list_books).post(create_book))
        .route(
            "/books/:id",
            get(get_book).put(update_book).delete(delete_book),
        )
        .route(
            "/protected",
            get(|UserId(id): UserId| async move {
                format!("Hello {}", id);
            }),
        )
}

async fn list_books(book_service: BookService) -> Json<Value> {
    let books: Vec<BookEntity> = book_service.list_books().await;
    Json(json!({
        "books": books,
    }))
}

async fn get_book(
    book_service: BookService,
    Path(book_id): Path<u32>,
) -> Result<Json<Value>, StatusCode> {
    let book: Option<BookEntity> = book_service.get_book(book_id).await;
    book.map(|book| {
        Json(json!({
            "book": book,
        }))
    })
    .ok_or(StatusCode::NOT_FOUND)
}

async fn create_book(
    book_service: BookService,
    Json(payload): Json<BookEntityForCreation>,
) -> Result<Json<Value>, StatusCode> {
    let book_id = book_service.create_book(payload).await;
    book_id
        .map(|book_id| {
            Json(json!({
                "book_id": book_id,
            }))
        })
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn update_book(
    book_service: BookService,
    Path(book_id): Path<u32>,
    Json(payload): Json<BookEntityForCreation>,
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
