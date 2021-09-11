use rocket::get;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, routes_with_openapi};

/// wao
/// dododo
/// pepepe
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[rocket::launch]
fn app() -> _ {
    rocket::build()
        .mount("/", routes_with_openapi![index])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
