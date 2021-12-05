use serde::{Deserialize, Serialize};

use super::Pid;

#[derive(Debug, Serialize)]
pub struct BookEntity {
    pub id: Pid,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct BookEntityForCreation {
    pub title: String,
}

impl From<(Pid, BookEntityForCreation)> for BookEntity {
    fn from((id, book): (u32, BookEntityForCreation)) -> BookEntity {
        Self {
            id,
            title: book.title,
        }
    }
}
