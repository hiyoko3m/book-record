use yew::prelude::*;
use yew_router::prelude::*;

use crate::organism::common::{Footer, Header};
use crate::routes::Route;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <>
            <Header />
            <div>
                <p>{ "Error" }</p>
                <Link<Route> to={Route::Home}>{ "go to top" }</Link<Route>>
            </div>
            <Footer />
        </>
    }
}
