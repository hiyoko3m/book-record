use rocket::serde::Serialize;

use crate::domain::entity::book::BookEntity;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    id: i32,
    title: String,
}

impl From<BookEntity> for Book {
    fn from(book_entity: BookEntity) -> Self {
        Self {
            id: book_entity.id,
            title: book_entity.title,
        }
    }
}
