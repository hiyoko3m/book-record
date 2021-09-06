use rocket;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn app() -> _ {
    rocket::build().mount("/", rocket::routes![index])
}
