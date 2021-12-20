use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::atom::common::{ButtonSecondary, ModalButtonSecondary};

#[derive(Clone, Default, PartialEq, Deserialize)]
pub struct Book {
    pub id: usize,
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct BooksProps {
    pub books: Vec<Book>,
    pub on_click: Callback<Book>,
    pub on_edit_click: Callback<Book>,
    pub delete_target: String,
    pub on_delete_click: Callback<Book>,
}

#[function_component(BooksList)]
pub fn books_list(
    BooksProps {
        books,
        on_click,
        on_edit_click,
        delete_target,
        on_delete_click,
    }: &BooksProps,
) -> Html {
    let on_click = on_click.clone();
    let on_edit_click = on_edit_click.clone();
    let on_delete_click = on_delete_click.clone();
    let books: Html = books
        .iter()
        .map(|book| {
            let on_book_click = {
                let on_click = on_click.clone();
                let book = book.clone();
                Callback::from(move |_| on_click.emit(book.clone()))
            };
            let on_edit_click = {
                let on_edit_click = on_edit_click.clone();
                let book = book.clone();
                Callback::from(move |_| on_edit_click.emit(book.clone()))
            };
            let on_delete_click = {
                let on_delete_click = on_delete_click.clone();
                let book = book.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_delete_click.emit(book.clone())
                })
            };

            html! {
                <>
                    <BookListItem title={book.title.clone()} on_click={on_book_click} on_edit_click={on_edit_click} on_delete_click={on_delete_click} delete_target={delete_target.clone()}/>
                </>
            }
        })
        .collect();

    html! {
        <div class="row row-cols-1 row-cols-md-2 g-4">
            { books }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BookItemProps {
    pub title: String,
    pub on_click: Callback<MouseEvent>,
    pub on_edit_click: Callback<MouseEvent>,
    pub on_delete_click: Callback<MouseEvent>,
    pub delete_target: String,
}

#[function_component(BookListItem)]
pub fn book_list_item(book: &BookItemProps) -> Html {
    html! {
        <div class="col">
            <div class="card book-list-item" onclick={book.on_click.clone()}>
                <div class="card-body">
                    <h5 class="card-title mb-3">{ book.title.to_owned() }</h5>
                    <ButtonSecondary message={"編集".to_string()} on_click={book.on_edit_click.clone()} additional_class={vec!["me-2".to_string()]}/>
                    <ModalButtonSecondary message={"削除".to_string()} on_click={book.on_delete_click.clone()} modal_target={book.delete_target.clone()}/>
                </div>
            </div>
        </div>
    }
}
