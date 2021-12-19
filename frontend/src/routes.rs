use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/books")]
    BookList,
    #[at("/books/:id")]
    BookDetail { id: usize },
    #[at("/books/:id/edit")]
    BookEdit { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}
