use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::molecule::book::BookListItem;

#[derive(Clone, Default, PartialEq, Deserialize)]
pub struct Book {
    pub id: usize,
    title: String,
}

#[derive(Properties, PartialEq)]
pub struct BooksProps {
    pub books: Vec<Book>,
    pub on_click: Callback<Book>,
    pub on_edit_click: Callback<Book>,
    pub on_delete_click: Callback<Book>,
}

#[function_component(BooksList)]
pub fn books_list(
    BooksProps {
        books,
        on_click,
        on_edit_click,
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
                    <BookListItem title={book.title.clone()} on_click={on_book_click} on_edit_click={on_edit_click} on_delete_click={on_delete_click} modal_target={"#exampleModal".to_string()}/>
                    <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                        <div class="modal-dialog">
                            <div class="modal-content">
                                <div class="modal-header">
                                    <h5 class="modal-title" id="exampleModalLabel">{ "Modal title" }</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                </div>
                                <div class="modal-body">{
                                    "..."
                                }</div>
                                <div class="modal-footer">
                                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{ "Close" }</button>
                                    <button type="button" class="btn btn-primary">{ "Save changes" }</button>
                                </div>
                            </div>
                        </div>
                    </div>
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
