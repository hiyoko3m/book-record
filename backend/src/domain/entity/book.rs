use serde::Serialize;

use super::PID;

#[derive(Debug, Serialize)]
pub struct BookEntity {
    pub id: PID,
    pub title: String,
}

#[derive(Debug)]
pub struct BookEntityForCreation {
    pub title: String,
}
