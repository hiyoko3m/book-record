use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct BookEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
