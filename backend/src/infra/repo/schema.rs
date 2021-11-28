use sqlx::FromRow;

use super::PID;
use crate::domain::entity::{self, book::BookEntity, user::UserEntity};

#[derive(FromRow)]
pub struct BookRow {
    id: PID,
    title: String,
}

impl From<BookRow> for BookEntity {
    fn from(book_row: BookRow) -> BookEntity {
        Self {
            id: book_row.id as entity::PID,
            title: book_row.title,
        }
    }
}

#[derive(FromRow)]
pub struct UserRow {
    id: PID,
    subject: String,
    username: String,
}

impl From<UserRow> for UserEntity {
    fn from(user_row: UserRow) -> UserEntity {
        Self {
            id: user_row.id as entity::PID,
            subject: user_row.subject,
            username: user_row.username,
        }
    }
}

#[derive(FromRow)]
pub struct UserIdRow {
    id: PID,
}

impl From<UserIdRow> for entity::PID {
    fn from(row: UserIdRow) -> entity::PID {
        row.id as entity::PID
    }
}
