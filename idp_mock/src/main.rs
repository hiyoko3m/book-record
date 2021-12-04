use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Form, Query, TypedHeader},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response::Html,
    routing::{get, post},
    AddExtensionLayer, Json, Router,
};
use chrono::{Duration, Utc};
use headers::{authorization::Basic, Authorization};
use openidconnect::core::{
    CoreClaimName, CoreIdToken, CoreIdTokenClaims, CoreIdTokenFields, CoreJsonWebKeySet,
    CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType, CoreRsaPrivateSigningKey,
    CoreSubjectIdentifierType, CoreTokenResponse, CoreTokenType,
};
use openidconnect::{
    AccessToken, Audience, AuthUrl, EmptyAdditionalClaims, EmptyAdditionalProviderMetadata,
    EmptyExtraTokenFields, EndUserEmail, IssuerUrl, JsonWebKeyId, JsonWebKeySetUrl, Nonce,
    PrivateSigningKey, ResponseTypes, Scope, StandardClaims, SubjectIdentifier, TokenUrl,
    UserInfoUrl,
};
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone)]
struct Settings {
    base_url: String,
    port: u16,
    rsa_pem: String,
}

async fn metadata(Extension(settings): Extension<Settings>) -> Json<CoreProviderMetadata> {
    let provider_metadata = CoreProviderMetadata::new(
        IssuerUrl::new(format!("{}:{}", settings.base_url, settings.port)).unwrap(),
        AuthUrl::new(format!("{}:{}/auth", settings.base_url, settings.port)).unwrap(),
        JsonWebKeySetUrl::new(format!("{}:{}/certs", settings.base_url, settings.port)).unwrap(),
        vec![ResponseTypes::new(vec![CoreResponseType::Code])],
        vec![CoreSubjectIdentifierType::Public],
        vec![CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256],
        EmptyAdditionalProviderMetadata {},
    )
    .set_token_endpoint(Some(
        TokenUrl::new(format!("{}:{}/token", settings.base_url, settings.port)).unwrap(),
    ))
    .set_userinfo_endpoint(Some(
        UserInfoUrl::new(format!("{}:{}/userinfo", settings.base_url, settings.port)).unwrap(),
    ))
    .set_scopes_supported(Some(vec![
        Scope::new("openid".to_string()),
        Scope::new("email".to_string()),
        Scope::new("profile".to_string()),
    ]))
    .set_claims_supported(Some(vec![
        CoreClaimName::new("aud".to_string()),
        CoreClaimName::new("email".to_string()),
        CoreClaimName::new("email_verified".to_string()),
        CoreClaimName::new("exp".to_string()),
        CoreClaimName::new("family_name".to_string()),
        CoreClaimName::new("given_name".to_string()),
        CoreClaimName::new("iat".to_string()),
        CoreClaimName::new("iss".to_string()),
        CoreClaimName::new("locale".to_string()),
        CoreClaimName::new("name".to_string()),
        CoreClaimName::new("picture".to_string()),
        CoreClaimName::new("sub".to_string()),
    ]));

    Json(provider_metadata)
}

async fn jwks(Extension(settings): Extension<Settings>) -> Json<CoreJsonWebKeySet> {
    let rsa_key = CoreRsaPrivateSigningKey::from_pem(
        &settings.rsa_pem,
        Some(JsonWebKeyId::new("key".to_string())),
    )
    .unwrap();
    let jwks = CoreJsonWebKeySet::new(vec![rsa_key.as_verification_key()]);
    Json(jwks)
}

#[derive(Debug, serde::Deserialize)]
struct TokenPayload {
    code: String,
    redirect_uri: String,
    grant_type: String,
    code_verifier: String,
}

async fn id_token(
    TypedHeader(Authorization(basic)): TypedHeader<Authorization<Basic>>,
    Extension(settings): Extension<Settings>,
    Extension(code_map): Extension<Arc<Mutex<HashMap<String, AuthInfo>>>>,
    Form(payload): Form<TokenPayload>,
) -> Result<Json<CoreTokenResponse>, StatusCode> {
    tracing::info!("{:?}", payload);

    let code_map = Arc::clone(&code_map);
    let code_map = code_map.lock().unwrap();
    let auth_info = code_map.get(&payload.code);
    if auth_info.is_none() {
        return Err(StatusCode::FORBIDDEN);
    }
    let auth_info = auth_info.unwrap();

    let rsa_key = CoreRsaPrivateSigningKey::from_pem(
        &settings.rsa_pem,
        Some(JsonWebKeyId::new("key".to_string())),
    )
    .unwrap();

    let id_token = CoreIdToken::new(
        CoreIdTokenClaims::new(
            IssuerUrl::new(format!("{}:{}", settings.base_url, settings.port)).unwrap(),
            vec![Audience::new(basic.username().to_string())],
            Utc::now() + Duration::seconds(300),
            Utc::now(),
            StandardClaims::new(SubjectIdentifier::new(auth_info.sub.to_owned()))
                .set_email(Some(EndUserEmail::new(auth_info.email.to_owned()))),
            EmptyAdditionalClaims {},
        )
        .set_nonce(Some(Nonce::new(auth_info.nonce.to_owned()))),
        &rsa_key,
        CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
        None,
        None,
    )
    .unwrap();

    Ok(Json(CoreTokenResponse::new(
        AccessToken::new("some_secret".to_string()),
        CoreTokenType::Bearer,
        CoreIdTokenFields::new(Some(id_token), EmptyExtraTokenFields {}),
    )))
}

#[derive(Debug, serde::Deserialize)]
struct AuthPayload {
    client_id: String,
    response_type: String,
    scope: String,
    redirect_uri: String,
    state: String,
    nonce: String,
    code_challenge: String,
    code_challenge_method: String, // S256のみとする
}

async fn auth_form(Query(payload): Query<AuthPayload>) -> Html<String> {
    Html(format!(
        "<html lang=\"ja\">
        <head><meta charset=\"utf-8\"></head>
        <body>
            <form action=\"/auth\" method=\"post\">
                <input type=\"hidden\" name=\"nonce\" value=\"{}\">
                <input type=\"hidden\" name=\"state\" value=\"{}\">
                <input type=\"hidden\" name=\"redirect_uri\" value=\"{}\">
                <label for=\"sub\">sub: </label>
                <input type=\"text\" id=\"sub\" name=\"sub\">
                <label for=\"email\">E-mail: </label>
                <input type=\"text\" id=\"email\" name=\"email\">
                <button type=\"submit\">認証</button>
            </form>
        </body>",
        payload.nonce, payload.state, payload.redirect_uri
    ))
}

#[derive(Debug, serde::Deserialize)]
struct RequirementPayload {
    sub: String,
    email: String,
    nonce: String,
    state: String,
    redirect_uri: String,
}

async fn auth(
    Form(payload): Form<RequirementPayload>,
    Extension(code_map): Extension<Arc<Mutex<HashMap<String, AuthInfo>>>>,
) -> (StatusCode, HeaderMap) {
    let code_map = Arc::clone(&code_map);
    tracing::info!("code_map before: {:?}", code_map.lock().unwrap());

    let auth_info = AuthInfo {
        sub: payload.sub,
        email: payload.email,
        nonce: payload.nonce,
    };
    let uuid = uuid::Uuid::new_v4();
    let code = format!("4/{}", uuid);
    let encoded_code = format!("4%2F{}", uuid);
    code_map.lock().unwrap().insert(code, auth_info);

    tracing::info!("code_map after: {:?}", code_map.lock().unwrap());

    let mut header_map = HeaderMap::new();
    header_map.insert(
            HeaderName::from_static("location"),
            HeaderValue::try_from(format!("{}/?state={}&code={}&scope=email+openid+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email&authuser=0&prompt=consent#", payload.redirect_uri, payload.state, encoded_code)).unwrap(),
    );

    (StatusCode::SEE_OTHER, header_map)
}

#[derive(Debug, Clone)]
struct AuthInfo {
    sub: String,
    email: String,
    nonce: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8001".to_string())
        .parse::<u16>()
        .unwrap();
    let rsa_pem_file = std::env::var("RSA_PEM_FILE").unwrap();
    let mut file = File::open(rsa_pem_file).unwrap();
    let mut rsa_pem = String::new();
    file.read_to_string(&mut rsa_pem).unwrap();
    let settings = Settings {
        base_url,
        port,
        rsa_pem,
    };

    let code_map: HashMap<String, AuthInfo> = HashMap::new();

    let app = Router::new()
        .route("/.well-known/openid-configuration", get(metadata))
        .route("/certs", get(jwks))
        .route("/token", post(id_token))
        .route("/auth", get(auth_form).post(auth))
        .layer(AddExtensionLayer::new(settings))
        .layer(AddExtensionLayer::new(Arc::new(Mutex::new(code_map))))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("server is listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
