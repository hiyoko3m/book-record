mod models;
mod schema;

// 本当はrocket_sync_db_poolsの中のdieselと被るので
// extern crateしたくないのだが、
// schema.rsのコンパイルを通すときには現実的な解はこれしかなさそう
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::{get, launch, routes};
use rocket_diesel::{QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::{database, diesel as rocket_diesel};

use self::models::Book;

#[database("db_book_record")]
struct BookRecordDbConn(rocket_diesel::MysqlConnection);

#[get("/")]
async fn index(conn: BookRecordDbConn) -> Json<Option<Book>> {
    use self::schema::books::dsl::*;

    let res = conn.run(|c| books.limit(1).load::<Book>(c)).await;
    Json(res.ok().map(|mut v| v.pop()).flatten())
}

#[launch]
fn app() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(BookRecordDbConn::fairing())
        .mount("/", routes![index])
}
