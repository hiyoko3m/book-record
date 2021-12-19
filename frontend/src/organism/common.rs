use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <nav class="navbar navbar-light bg-light">
                <div class="container-lg">
                    <Link<Route> to={Route::Home} classes="navbar-brand">{ "Navbar" }</Link<Route>>
                    <div class="d-flex">
                        <button class="btn" type="submit">{ "tmp" }</button>
                    </div>
                </div>
            </nav>
        </header>
    }
}

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mt-5 p-5 bg-light text-center">
            <div class="container-lg">
                { "footer" }
            </div>
        </footer>
    }
}
