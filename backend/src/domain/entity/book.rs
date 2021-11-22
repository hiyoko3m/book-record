use serde::{Deserialize, Serialize};

use super::PID;

#[derive(Debug, Serialize)]
pub struct BookEntity {
    pub id: PID,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct BookEntityForCreation {
    pub title: String,
}

impl From<(PID, BookEntityForCreation)> for BookEntity {
    fn from((id, book): (u32, BookEntityForCreation)) -> BookEntity {
        Self {
            id: id,
            title: book.title,
        }
    }
}
