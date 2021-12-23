use yew::prelude::*;
use yew_router::prelude::*;

use crate::atom::common::SectionTitle;
use crate::organism::common::{Footer, Header};
use crate::routes::Route;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    html! {
        <>
            <Header />
            <div class="container-lg">
                <SectionTitle title={"エラー"} />
                <p class="mt-2">{ "指定されたページは存在しません" }</p>
                <Link<Route> to={Route::Home}>{ "トップページに戻る" }</Link<Route>>
            </div>
            <Footer />
        </>
    }
}
