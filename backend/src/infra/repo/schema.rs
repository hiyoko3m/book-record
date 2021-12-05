use sqlx::FromRow;

use super::Pid;
use crate::domain::entity::{self, book::BookEntity, user::UserEntity};

#[derive(FromRow)]
pub struct BookRow {
    id: Pid,
    title: String,
}

impl From<BookRow> for BookEntity {
    fn from(book_row: BookRow) -> BookEntity {
        Self {
            id: book_row.id as entity::Pid,
            title: book_row.title,
        }
    }
}

#[derive(FromRow)]
pub struct UserRow {
    id: Pid,
    subject: String,
    username: String,
}

impl From<UserRow> for UserEntity {
    fn from(user_row: UserRow) -> UserEntity {
        Self {
            id: user_row.id as entity::Pid,
            subject: user_row.subject,
            username: user_row.username,
        }
    }
}

#[derive(FromRow)]
pub struct UserIdRow {
    id: Pid,
}

impl From<UserIdRow> for entity::Pid {
    fn from(row: UserIdRow) -> entity::Pid {
        row.id as entity::Pid
    }
}
