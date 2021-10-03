use rocket::{get, launch, routes};
use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_book_record")]
struct BookRecordDbConn(diesel::SqliteConnection);

/// pepepe
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn app() -> _ {
    rocket::build()
        .attach(BookRecordDbConn::fairing())
        .mount("/", routes![index])
}
