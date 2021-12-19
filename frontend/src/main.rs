mod atom;
mod molecule;
mod organism;
mod page;
mod routes;

use yew::prelude::*;
use yew_router::prelude::*;

use self::page::{
    book::{BooksDetail, BooksListPage},
    error::NotFoundPage,
    home::HomePage,
};
use self::routes::Route;

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
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
