use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::atom::common::{ButtonSecondary, ModalButtonSecondary};

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Book {
    pub id: usize,
    pub title: String,
}

#[derive(Clone, Default, PartialEq, Serialize)]
pub struct BookForSend {
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
        <div class="row row-cols-1 row-cols-md-2 g-3 mb-3">
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

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateBookFormProps {
    pub on_submit: Callback<BookForSend>,
}

#[function_component(CreateBookForm)]
pub fn create_book_form(CreateBookFormProps { on_submit }: &CreateBookFormProps) -> Html {
    let title_ref = use_node_ref();
    let on_create = {
        let title_ref = title_ref.clone();
        let on_submit = on_submit.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();

            if let Some(title_input) = title_ref.cast::<HtmlInputElement>() {
                let book = BookForSend {
                    title: title_input.value(),
                };
                on_submit.emit(book);
                title_input.set_value("");
            }
        })
    };

    html! {
        <>
            <button class="btn d-inline-flex align-item-center lh-1 fold-button collapsed" data-bs-toggle="collapse" data-bs-target="#create-book-form" aria-expanded="true" aria-current="true">{"新しい本の追加"}</button>
            <div class="collapse" id="create-book-form">
                <div class="mx-2 mt-3 p-3 border border-2">
                    <form class="row">
                        <label for="title" class="col-sm-2 col-form-label">{ "タイトル" }</label>
                        <div class="col-sm-10">
                            <input ref={title_ref} type="text" class="form-control" id="title" />
                        </div>
                        <div class="col-12 mt-3">
                            <button type="submit" class="btn btn-primary" onclick={on_create}>{ "作成" }</button>
                        </div>
                    </form>
                </div>
            </div>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EditBookFormProps {
    pub on_submit: Callback<BookForSend>,
    pub book: Book,
}

#[function_component(EditBookForm)]
pub fn edit_book_form(EditBookFormProps { on_submit, book }: &EditBookFormProps) -> Html {
    let title_ref = use_node_ref();
    let on_edit = {
        let title_ref = title_ref.clone();
        let on_submit = on_submit.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();

            if let Some(title_input) = title_ref.cast::<HtmlInputElement>() {
                let book = BookForSend {
                    title: title_input.value(),
                };
                on_submit.emit(book);
            }
        })
    };

    html! {
        <>
                <div class="mx-2 mt-3 p-3 border border-2">
                    <form class="row">
                        <label for="title" class="col-sm-2 col-form-label">{ "タイトル" }</label>
                        <div class="col-sm-10">
                            <input ref={title_ref} type="text" class="form-control" id="title" value={ book.clone().title } />
                        </div>
                        <div class="col-12 mt-3">
                            <button type="submit" class="btn btn-primary" onclick={on_edit}>{ "編集" }</button>
                        </div>
                    </form>
                </div>
        </>
    }
}
