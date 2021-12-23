mod atom;
mod organism;
mod page;
mod routes;
mod settings;

use dotenv_codegen::dotenv;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use self::page::{
    book::{BooksDetailPage, BooksEditPage, BooksListPage},
    error::NotFoundPage,
    home::HomePage,
};
use self::routes::Route;
use self::settings::Settings;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::BookList => html! { <BooksListPage /> },
        Route::BookDetail { id } => html! { <BooksDetailPage id={id.clone()} /> },
        Route::BookEdit { id } => html! { <BooksEditPage id={id.clone() } /> },
        _ => html! { <NotFoundPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let base_url = dotenv!("BASE_URL");
    let settings = use_state(|| Settings {
        base_url: Url::parse(base_url).unwrap(),
    });

    html! {
        <BrowserRouter>
            <ContextProvider<Settings> context={(*settings).clone()}>
                <Switch<Route> render={Switch::render(switch)} />
            </ContextProvider<Settings>>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
