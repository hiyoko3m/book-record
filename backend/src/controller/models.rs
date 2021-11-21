use serde::Deserialize;

use crate::domain::entity::book::{BookEntity, BookEntityForCreation};

#[derive(Debug, Deserialize)]
pub struct BookExtract {
    title: String,
}

impl From<BookExtract> for BookEntityForCreation {
    fn from(book_extract: BookExtract) -> BookEntityForCreation {
        Self {
            title: book_extract.title,
        }
    }
}

impl From<(u32, BookExtract)> for BookEntity {
    fn from((id, book_extract): (u32, BookExtract)) -> BookEntity {
        Self {
            id: id,
            title: book_extract.title,
        }
    }
}
