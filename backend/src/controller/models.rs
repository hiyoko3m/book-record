use serde::Deserialize;

use crate::domain::entity::book::{BookEntity, BookEntityForCreate};

#[derive(Debug, Deserialize)]
pub struct BookExtract {
    title: String,
}

impl From<BookExtract> for BookEntityForCreate {
    fn from(book_extract: BookExtract) -> BookEntityForCreate {
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
