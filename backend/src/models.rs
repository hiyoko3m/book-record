use rocket::serde::Serialize;
use rocket_sync_db_pools::diesel::Queryable;

#[derive(Debug, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Book {
    pub id: i32,
    pub title: String,
}
