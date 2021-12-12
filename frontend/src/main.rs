use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/books")]
    ListBooks,
    #[at("/books/:id")]
    DescribeBook { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Deserialize)]
struct BooksResponse {
    books: Vec<Book>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Book {
    id: usize,
    title: String,
}

#[derive(Properties, PartialEq)]
struct BooksProps {
    books: Vec<Book>,
    on_click: Callback<Book>,
}

#[function_component(BooksList)]
fn books_list(BooksProps { books, on_click }: &BooksProps) -> Html {
    books
        .iter()
        .map(|book| {
            let on_book_select = {
                let on_click = on_click.clone();
                let book = book.clone();
                Callback::from(move |_| on_click.emit(book.clone()))
            };

            html! {
                <p onclick={on_book_select}>{ format!("Book {}", book.title) }</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct BookDetailsProps {
    book: Book,
}

#[function_component(BookDetails)]
fn book_details(BookDetailsProps { book }: &BookDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ book.title.clone() }</h3>
            <p>{ "uwaaaaaaaaaa" }</p>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    unimplemented!();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
    //    let books = use_state(|| vec![]);
    //    {
    //        let books = books.clone();
    //        use_effect_with_deps(
    //            move |_| {
    //                let books = books.clone();
    //                wasm_bindgen_futures::spawn_local(async move {
    //                    let response: BooksResponse = Request::get("http://localhost:8000/v1/books")
    //                        .send()
    //                        .await
    //                        .unwrap()
    //                        .json()
    //                        .await
    //                        .unwrap();
    //                    books.set(response.books);
    //                });
    //                || ()
    //            },
    //            (),
    //        );
    //    }
    //
    //    let selected_book = use_state(|| None);
    //    let on_book_select = {
    //        let selected_book = selected_book.clone();
    //        Callback::from(move |book: Book| selected_book.set(Some(book)))
    //    };
    //
    //    let details = selected_book.as_ref().map(|book| {
    //        html! {
    //            <div class="row justify-content-center">
    //                <div class="col-6">
    //                    <BookDetails book={book.clone()} />
    //                </div>
    //            </div>
    //        }
    //    });
    //
    //    html! {
    //        <>
    //            <div class="container">
    //                <BooksList books={(*books).clone()} on_click={on_book_select.clone()} />
    //                { for details }
    //            </div>
    //        </>
    //    }
}

fn main() {
    yew::start_app::<App>();
}
