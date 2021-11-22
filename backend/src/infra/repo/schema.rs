use sqlx::FromRow;

use super::PID;
use crate::domain::entity::book::BookEntity;
use crate::domain::entity::PID as EPID;

#[derive(FromRow)]
pub struct BookRow {
    id: PID,
    title: String,
}

impl From<BookRow> for BookEntity {
    fn from(book_row: BookRow) -> BookEntity {
        Self {
            id: book_row.id as EPID,
            title: book_row.title,
        }
    }
}

#[derive(FromRow)]
pub struct UserRow {
    id: PID,
    sub: String,
    username: String,
}
