use yew::prelude::*;

use crate::atom::common::{ButtonSecondary, ModalButtonSecondary};

#[derive(Properties, PartialEq)]
pub struct BookItemProps {
    pub title: String,
    pub on_click: Callback<MouseEvent>,
    pub on_edit_click: Callback<MouseEvent>,
    pub on_delete_click: Callback<MouseEvent>,
    pub modal_target: String,
}

#[function_component(BookListItem)]
pub fn book_list_item(book: &BookItemProps) -> Html {
    html! {
        <div class="col">
            <div class="card book-list-item" onclick={book.on_click.clone()}>
                <div class="card-body">
                    <h5 class="card-title mb-3">{ book.title.to_owned() }</h5>
                    <ButtonSecondary message={"編集".to_string()} on_click={book.on_edit_click.clone()} additional_class={vec!["me-2".to_string()]}/>
                    <ModalButtonSecondary message={"削除".to_string()} on_click={book.on_delete_click.clone()} modal_target={book.modal_target.clone()}/>
                </div>
            </div>
        </div>
    }
}
