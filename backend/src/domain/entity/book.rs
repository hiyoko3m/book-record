use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BookEntity {
    pub id: u32,
    pub title: String,
}

#[derive(Debug)]
pub struct BookEntityForCreate {
    pub title: String,
}
