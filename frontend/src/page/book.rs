use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::atom::common::SectionTitle;
use crate::organism::book::{Book, BookForSend, BooksList, CreateBookForm, EditBookForm};
use crate::organism::common::{DeleteModal, Footer, Header};
use crate::routes::Route;
use crate::settings::Settings;

#[derive(Deserialize)]
struct BooksResponse {
    books: Vec<Book>,
}

#[function_component(BooksListPage)]
pub fn list_books() -> Html {
    let settings = use_context::<Settings>().expect("settings context cannot be found");
    let history = use_history().expect("history API encounters a critical error");

    let books = use_state(|| vec![]);

    let fetch_books = {
        let settings = settings.clone();
        |books: UseStateHandle<Vec<Book>>| async move {
            let response: BooksResponse =
                Request::get(settings.base_url.join("books").unwrap().as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            books.set(response.books);
        }
    };

    {
        let books = books.clone();
        let fetch_books = fetch_books.clone();
        use_effect_with_deps(
            move |_| {
                let books = books.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    fetch_books(books).await;
                });
                || ()
            },
            (),
        );
    }

    let on_click = {
        let history = history.clone();
        Callback::once(move |book: Book| history.push(Route::BookDetail { id: book.id }))
    };
    let on_edit_click = {
        let history = history.clone();
        Callback::once(move |book: Book| history.push(Route::BookEdit { id: book.id }))
    };

    let delete_target = use_state(|| Book::default());

    // アイテム一覧内の削除ボタンが押されたときのコールバック
    // まだ確認モーダルを出すだけで、実際の削除はしない
    let on_delete_click = {
        let delete_target = delete_target.clone();
        Callback::from(move |book: Book| delete_target.set(book))
    };

    let delete_modal_id = "deleteModal".to_string();
    let delete_modal_message = format!("{} を本当に削除しますか？", (*delete_target).title);

    let on_delete_confirmed = {
        let delete_target = delete_target.clone();
        let settings = settings.clone();
        let books = books.clone();
        let fetch_books = fetch_books.clone();
        Callback::from(move |_| {
            let delete_target = delete_target.clone();
            let settings = settings.clone();
            let books = books.clone();
            let fetch_books = fetch_books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _response = Request::delete(
                    settings
                        .base_url
                        .join(format!("books/{}", (*delete_target).id).as_str())
                        .unwrap()
                        .as_str(),
                )
                .send()
                .await
                .unwrap();
                fetch_books(books).await;
            });
        })
    };

    let on_submit = {
        let settings = settings.clone();
        let books = books.clone();
        let fetch_books = fetch_books.clone();
        Callback::from(move |book: BookForSend| {
            let settings = settings.clone();
            let books = books.clone();
            let fetch_books = fetch_books.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let _response = Request::post(settings.base_url.join("books").unwrap().as_str())
                    .body(serde_json::json!(book).to_string())
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();
                fetch_books(books).await;
            });
        })
    };

    html! {
        <>
            <Header />
            <div class="container-lg">
                <SectionTitle title={"登録された本の一覧".to_string()} />
                <BooksList books={(*books).clone()} {on_click} {on_edit_click} {on_delete_click} delete_target={format!("#{}", delete_modal_id)}/>
                <CreateBookForm {on_submit}/>
            </div>
            <DeleteModal title={"確認".to_string()} message={delete_modal_message.clone()} id={delete_modal_id.clone()} label={"deleteModalLabel".to_string()} on_click={on_delete_confirmed} />
            <Footer />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct BooksDetailProps {
    pub id: usize,
}

#[derive(Deserialize)]
struct BooksDetailResponse {
    book: Book,
}

#[function_component(BooksDetailPage)]
pub fn describe_book(props: &BooksDetailProps) -> Html {
    let settings = use_context::<Settings>().expect("settings context cannot be found");

    let book = use_state(|| None);

    let fetch_book = {
        let settings = settings.clone();
        |id, book: UseStateHandle<Option<Book>>| async move {
            let response: BooksDetailResponse = Request::get(
                settings
                    .base_url
                    .join(&format!("books/{}", id))
                    .unwrap()
                    .as_str(),
            )
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
            book.set(Some(response.book));
        }
    };

    {
        let book = book.clone();
        let id = props.id;
        use_effect_with_deps(
            move |_| {
                let book = book.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    fetch_book(id, book).await;
                });
                || ()
            },
            (),
        );
    }

    let history = use_history().expect("history API encounters a critical error");
    let on_back_click = Callback::once(move |_| history.back());

    let content = if let Some(book) = (*book).clone() {
        html! { <SectionTitle title={ book.title } /> }
    } else {
        html! { <div>{ "読み込み中" } </div> }
    };

    html! {
        <>
            <Header />
            <div class="container-lg">
                { content }
                <a href="#" onclick={on_back_click}>{ "前のページに戻る" }</a>
            </div>
            <Footer />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct BooksEditProps {
    pub id: usize,
}

#[function_component(BooksEditPage)]
pub fn edit_book(props: &BooksEditProps) -> Html {
    let settings = use_context::<Settings>().expect("settings context cannot be found");

    let book = use_state(|| None);

    {
        let book = book.clone();
        let id = props.id;
        let settings = settings.clone();
        use_effect_with_deps(
            move |_| {
                let book = book.clone();
                let settings = settings.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response: BooksDetailResponse = Request::get(
                        settings
                            .base_url
                            .join(&format!("books/{}", id))
                            .unwrap()
                            .as_str(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                    book.set(Some(response.book));
                });
                || ()
            },
            (),
        );
    }

    let history = use_history().expect("history API encounters a critical error");
    let on_back_click = {
        let history = history.clone();
        Callback::once(move |_| history.back())
    };

    let content = if let Some(book) = (*book).clone() {
        let on_click = {
            let settings = settings.clone();
            let history = history.clone();
            let id = props.id;
            Callback::once(move |book| {
                let settings = settings.clone();
                let history = history.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _response = Request::put(
                        settings
                            .base_url
                            .join(&format!("books/{}", id))
                            .unwrap()
                            .as_str(),
                    )
                    .body(serde_json::json!(book).to_string())
                    .header("content-type", "application/json")
                    .send()
                    .await
                    .unwrap();
                    history.push(Route::BookList);
                });
            })
        };

        html! {
            <>
                <SectionTitle title={ book.clone().title } />
                <EditBookForm book={book.clone()} on_submit={on_click} />
            </>
        }
    } else {
        html! { <div>{ "読み込み中" } </div> }
    };

    html! {
        <>
            <Header />
            <div class="container-lg">
                { content }
                <a href="#" class="d-block mt-2" onclick={on_back_click}>{ "前のページに戻る" }</a>
            </div>
            <Footer />
        </>
    }
}
