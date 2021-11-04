use serde::Serialize;

use crate::domain::entity::book::BookEntity;

#[derive(Debug, Serialize)]
pub struct Book {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

impl From<BookEntity> for Book {
    fn from(book_entity: BookEntity) -> Self {
        Self {
            id: book_entity.id,
            title: book_entity.title,
            body: book_entity.body,
            published: book_entity.published,
        }
    }
}
