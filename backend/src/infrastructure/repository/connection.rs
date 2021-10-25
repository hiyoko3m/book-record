use rocket_sync_db_pools::{database, diesel};

#[database("db_book_record")]
pub struct BookRecordMysqlConn(diesel::MysqlConnection);
