mod controller;
mod domain;
mod infra;
mod settings;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{AddExtensionLayer, Router};
use dotenv::dotenv;
use openidconnect::core::{CoreClient, CoreProviderMetadata};
use openidconnect::reqwest::async_http_client;
use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
use sqlx::postgres::PgPool;

use self::controller::{book::book_app, user::user_app};
use self::settings::Settings;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("initialization start");

    // .envファイル読み込み
    dotenv().ok();

    // 設定変数の初期化
    let settings = envy::from_env::<Settings>().expect(
        "initialization error: failed in constructing app's settings from environment variables",
    );

    // repository層の外部アクセス先の初期化
    let pool = PgPool::connect(&settings.database_url).await.unwrap();
    let redis_cli = redis::Client::open(settings.redis_url.to_owned())
        .expect("initialization error: connecting Redis server failed");

    // IdPの設定初期化
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(settings.id_provider_url.to_owned())
            .expect("initialization error: failed in parsing the URL of IdP"),
        async_http_client,
    )
    .await
    .expect("initialization error: failed in discovering the IdP's metadata");

    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(settings.id_provider_client_id.to_owned()),
        Some(ClientSecret::new(
            settings.id_provider_client_secret.to_owned(),
        )),
    )
    .set_redirect_uri(
        RedirectUrl::new(settings.id_provider_redirect_url.to_owned()).expect(
            "initialization error: failed in parsing the redirect URL which be passed to IdP",
        ),
    );

    // axumのアプリケーション構築
    let app = Router::new().nest(
        "/v1",
        Router::new()
            .merge(book_app())
            .merge(user_app())
            .layer(AddExtensionLayer::new(Arc::new(settings)))
            .layer(AddExtensionLayer::new(client))
            .layer(AddExtensionLayer::new(pool)),
        //.layer(AddExtensionLayer::new(redis_cli)),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    tracing::info!(
        "initialization complete; now the server is listening on {}",
        addr
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
