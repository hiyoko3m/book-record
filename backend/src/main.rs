mod controller;
mod domain;
mod infrastructure;
mod utils;

use std::net::SocketAddr;

use axum::{AddExtensionLayer, Router};
use sqlx::postgres::PgPool;

use self::controller::{book::book_app, user::user_app};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = PgPool::connect("postgres://hyo:hyo@localhost/book_record")
        .await
        .unwrap();

    let app = Router::new().nest(
        "/v1",
        Router::new()
            .merge(book_app())
            .merge(user_app())
            .layer(AddExtensionLayer::new(pool)),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
