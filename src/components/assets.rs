use crate::components::book::*;
use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn AssetsPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="mx-auto max-w-screen-xl px-4 gap-8">
            <div >
                <BookStorage/>
            </div>
            <div >
                <BookList />
            </div>
        </div>
    }
}
