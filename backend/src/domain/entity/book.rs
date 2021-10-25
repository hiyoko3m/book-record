use rocket_sync_db_pools::diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct BookEntity {
    pub id: i32,
    pub title: String,
}
