use sqlx::FromRow;

use crate::domain::entity::book::BookEntity;

#[derive(FromRow)]
pub struct BookRow {
    id: i32,
    title: String,
}

impl From<BookRow> for BookEntity {
    fn from(book_row: BookRow) -> BookEntity {
        Self {
            id: book_row.id as u32,
            title: book_row.title,
        }
    }
}
