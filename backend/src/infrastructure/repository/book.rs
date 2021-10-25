use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_sync_db_pools::diesel::RunQueryDsl;

use super::connection::BookRecordMysqlConn;
use crate::domain::entity::book::BookEntity;
use crate::domain::repository_interface::book::BookRepositoryInterface;

pub struct BookRepository {
    conn: BookRecordMysqlConn,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BookRepository {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let conn = try_outcome!(req.guard::<BookRecordMysqlConn>().await);
        Outcome::Success(Self { conn })
    }
}

#[rocket::async_trait]
impl BookRepositoryInterface for BookRepository {
    async fn list_books(&self) -> Vec<BookEntity> {
        use super::schema::books::dsl::*;
        let res = self.conn.run(|c| books.load::<BookEntity>(c)).await;
        res.unwrap_or(vec![])
    }
}
