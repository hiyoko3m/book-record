mod controller;
mod domain;
mod infrastructure;
mod utils;

#[macro_use]
extern crate diesel;

use axum::AddExtensionLayer;
use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use diesel::pg::PgConnection;

use self::controller::book::book_app;

#[tokio::main]
async fn main() {
    let manager = DieselConnectionManager::<PgConnection>::new("localhost:5432");
    let pool = Pool::builder().build(manager).await.unwrap();

    let app = book_app().layer(AddExtensionLayer::new(pool));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
