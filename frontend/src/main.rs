mod atom;
mod organism;
mod page;
mod routes;
mod settings;

use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use self::page::{
    book::{BooksDetail, BooksListPage},
    error::NotFoundPage,
    home::HomePage,
};
use self::routes::Route;
use self::settings::Settings;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::BookList => html! { <BooksListPage /> },
        Route::BookDetail { id } => html! { <BooksDetail id={id.clone()} /> },
        _ => html! { <NotFoundPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let settings = use_state(|| Settings {
        base_url: Url::parse("https://book-record-hiyoko3m.herokuapp.com/v1/").unwrap(),
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
