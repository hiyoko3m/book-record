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

#[derive(Properties, PartialEq)]
pub struct DeleteModalProps {
    pub title: String,
    pub message: String,
    pub id: String,
    pub label: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(DeleteModal)]
pub fn delete_modal(
    DeleteModalProps {
        title,
        message,
        id,
        label,
        on_click,
    }: &DeleteModalProps,
) -> Html {
    html! {
        <div class="modal fade" id={id.clone()} tabindex="-1" aria-labelledby={label.clone()} aria-hidden="true">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id={label.clone()}>{ title.clone() }</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">{message.clone()}</div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-danger" data-bs-dismiss="modal" onclick={on_click}>{ "削除" }</button>
                        <button type="button" class="btn btn-outline-secondary" data-bs-dismiss="modal">{ "キャンセル" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
