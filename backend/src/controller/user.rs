use axum::{
    extract::{Extension, TypedHeader},
    response::{Headers, IntoResponse},
    routing::post,
    Json, Router,
};
use headers::Cookie;
use serde_json::{json, Value};

use crate::controller::models::{LoginExtract, SignUpExtract};
use crate::domain::entity::user::{
    AccessToken, LoginError, RefreshToken, RefreshTokenError, RefreshTokenExtract, SignUpError,
};
use crate::domain::service::user::UserService;
use crate::settings::Settings;

pub fn user_app() -> Router {
    Router::new()
        .route("/login-session", post(make_login_session))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/token", post(refresh_tokens))
}

async fn make_login_session(user_service: UserService) -> Result<Json<Value>, LoginError> {
    user_service.make_login_session().await.map(|session| {
        Json(json!({
            "session_id": session.session_id,
            "nonce": session.nonce,
            "code_challenge": session.code_challenge.as_str().to_string(),
        }))
    })
}

fn response_from_tokens(
    cookie_name: &str,
    refresh_token: RefreshToken,
    access_token: AccessToken,
) -> impl IntoResponse {
    (
        Headers(vec![(
            "Set-Cookie",
            refresh_token.into_cookie_value(cookie_name, "/token"),
        )]),
        access_token.0,
    )
}

async fn login(
    user_service: UserService,
    Json(payload): Json<LoginExtract>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, LoginError> {
    user_service
        .login(payload.session_id, payload.code)
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}

async fn sign_up(
    user_service: UserService,
    Json(payload): Json<SignUpExtract>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, SignUpError> {
    user_service
        .sign_up(payload.code, payload.user)
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}

async fn refresh_tokens(
    user_service: UserService,
    cookie: Option<TypedHeader<Cookie>>,
    Extension(settings): Extension<Settings>,
) -> Result<impl IntoResponse, RefreshTokenError> {
    let refresh_token_value = if let Some(TypedHeader(cookie)) = cookie {
        cookie
            .get(&settings.refresh_token_cookie_name)
            .ok_or(RefreshTokenError::InvalidRefreshToken)?
            .to_owned()
    } else {
        return Err(RefreshTokenError::InvalidRefreshToken);
    };

    user_service
        .refresh_tokens(RefreshTokenExtract(refresh_token_value.to_string()))
        .await
        .map(|ts| response_from_tokens(&settings.refresh_token_cookie_name, ts.0, ts.1))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::{
        body::Body,
        http::{self, Method, Request, StatusCode},
        AddExtensionLayer,
    };
    use openidconnect::core::{CoreClient, CoreProviderMetadata};
    use openidconnect::reqwest::async_http_client;
    use openidconnect::{ClientId, ClientSecret, IssuerUrl, RedirectUrl};
    use sqlx::postgres::PgPool;
    use tower::ServiceExt;

    use super::*;
    use crate::settings::Settings;

    async fn test_app() -> Router {
        let settings = envy::from_env::<Settings>().unwrap();

        assert_eq!(settings.refresh_exp, 20);

        // repository層の外部アクセス先の初期化
        let pg_pool = PgPool::connect(&settings.database_url).await.unwrap();
        let redis_cli = redis::Client::open(settings.redis_url.to_owned()).unwrap();

        // IdPの設定初期化
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(settings.id_provider_url.to_owned())
                .expect("initialization error: failed in parsing the URL of IdP"),
            async_http_client,
        )
        .await
        .unwrap();

        let id_cli = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(settings.id_provider_client_id.to_owned()),
            Some(ClientSecret::new(
                settings.id_provider_client_secret.to_owned(),
            )),
        )
        .set_redirect_uri(RedirectUrl::new(settings.id_provider_redirect_url.to_owned()).unwrap());

        user_app()
            .layer(AddExtensionLayer::new(settings))
            .layer(AddExtensionLayer::new(id_cli))
            .layer(AddExtensionLayer::new(pg_pool))
            .layer(AddExtensionLayer::new(redis_cli))
    }

    // ログイン～ユーザ作成の成功シナリオ
    #[tokio::test]
    #[ignore]
    async fn login() {
        let app = test_app().await;

        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body["code_challenge"].is_string());
        assert!(body["nonce"].is_string());
        assert!(body["session_id"].is_string());

        // 後段で使うレスポンス
        let _code_challenge = body["code_challenge"].as_str().unwrap();
        let nonce = body["nonce"].as_str().unwrap();
        let session_id = body["session_id"].as_str().unwrap();

        // IdPモックに依存するログイン処理
        let state = "dummy";
        let redirect_uri = "http://localhost:8000";
        let subject = "testsub";
        let email = "test@example.jp";
        let params = [
            ("nonce", nonce),
            ("state", state),
            ("redirect_uri", redirect_uri),
            ("sub", subject),
            ("email", email),
        ];

        let reqwest_cli = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let res = reqwest_cli
            .post("http://localhost:8001/auth")
            .form(&params)
            .send()
            .await
            .unwrap();
        let url = reqwest::Url::parse(res.headers()[reqwest::header::LOCATION].to_str().unwrap())
            .unwrap();
        let pairs = url
            .query_pairs()
            .into_owned()
            .collect::<HashMap<String, String>>();

        // 後段で使うレスポンス
        let code = &pairs["code"];

        // codeを使ってログイン処理
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"session_id": session_id, "code": code}))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // ユーザがいないのでsign upコードが返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body["error"].is_string());

        // 後段で使うレスポンス
        let code = body["error"].as_str().unwrap();

        // session_idは使い捨てなので、
        // 同じsession_idを使うことはできない
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"session_id": session_id, "code": code}))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert!(body["error"].is_string());
        assert_eq!(body["error"].as_str().unwrap(), "");

        let username = "testname";

        // codeを使ってsign up処理
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"code": code, "user": {"username": username}}))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // ユーザ作成成功
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key(hyper::header::SET_COOKIE));
        let set_cookie = &response.headers()[hyper::header::SET_COOKIE];
        let refresh_token = set_cookie
            .to_str()
            .unwrap()
            .split(|a| a == ';')
            .next()
            .unwrap()
            .to_owned();
        // クッキーを設定していることを確認
        assert_ne!(refresh_token, "");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // アクセストークン
        // 空文字ではないことだけチェック
        let access_token = String::from_utf8_lossy(&body);
        assert_ne!(access_token, "");

        // 同じcodeを使ってsign upすることはできない
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"code": code, "user": {"username": username}}))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // ユーザ作成失敗
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        // refreshトークンを使って、トークンのリフレッシュ
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .header(http::header::COOKIE, refresh_token.to_owned())
                    .uri("/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        // 新しいrefreshトークンが届く
        assert!(response.headers().contains_key(hyper::header::SET_COOKIE));

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        // アクセストークンも届いている
        assert_ne!(String::from_utf8_lossy(&body), "");

        // 同じrefreshトークンを使うことはできない
        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .header(http::header::COOKIE, refresh_token)
                    .uri("/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    // /login に無効なsession_idを送る
    #[tokio::test]
    #[ignore]
    async fn login_nonexistent_session() {
        let app = test_app().await;

        // 無効なsession_id
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(
                            &json!({"session_id": "nonexistent", "code": "invalid"}),
                        )
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 403が返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        // 空文字が返ってくる
        assert!(body["error"].is_string());
        assert_eq!(body["error"].as_str().unwrap(), "");
    }

    // /login に有効なsession_idと無効なコードを送る
    #[tokio::test]
    #[ignore]
    async fn login_invalid_code() {
        let app = test_app().await;

        let response = app
            .to_owned()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login-session")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        // 後段で使うレスポンス
        let session_id = body["session_id"].as_str().unwrap();

        // 無効なコード
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/login")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"session_id": session_id, "code": "invalid"}))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 無効なコードでは403が返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        // 無効なコードでは空文字が返ってくる
        assert!(body["error"].is_string());
        assert_eq!(body["error"].as_str().unwrap(), "");
    }

    // /signup に無効なcodeを送る
    #[tokio::test]
    #[ignore]
    async fn signup_nonexistent_code() {
        let app = test_app().await;

        // 無効なcode
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/signup")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_vec(
                            &json!({"code": "nonexistent", "user": {"username": "dummy"}}),
                        )
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 403が返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }

    // /token にクッキーなしでアクセス
    #[tokio::test]
    #[ignore]
    async fn refresh_token_with_no_cookie() {
        let app = test_app().await;

        // クッキーなし
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 403が返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }

    // /token に無効なクッキーでアクセス
    #[tokio::test]
    #[ignore]
    async fn refresh_token_with_invalid_cookie() {
        let app = test_app().await;

        // 無効なクッキー
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/token")
                    .header(http::header::COOKIE, "refresh_token=invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // 403が返ってくる
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
