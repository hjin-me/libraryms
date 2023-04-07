use crate::components::book::*;
use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn AssetsPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="mx-auto max-w-screen-xl px-4 grid grid-cols-5 items-start gap-8">
            <div class="col-span-1">
                <BookStorage/>
            </div>
            <div class="col-span-4">
                <BookList />
            </div>
        </div>
    }
}
