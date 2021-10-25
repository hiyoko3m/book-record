mod controller;
mod domain;
mod infrastructure;

// 本当はrocket_sync_db_poolsの中のdieselと被るので
// extern crateしたくないのだが、
// schema.rsのコンパイルを通すときには現実的な解はこれしかなさそう
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::{launch, routes};

use self::controller::book::list_books;
use self::infrastructure::repository::connection::BookRecordMysqlConn;

#[launch]
fn app() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(BookRecordMysqlConn::fairing())
        .mount("/", routes![list_books])
}
