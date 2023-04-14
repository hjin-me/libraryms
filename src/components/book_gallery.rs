use crate::api::books::BookUI;
use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn BookGallery(cx: Scope, books: Vec<BookUI>) -> impl IntoView {
    view! {
        cx,
        <div class="mx-auto max-w-2xl px-4 my-4 lg:max-w-7xl">
        <h2 class="sr-only">"Books"</h2>

        <div class="grid grid-cols-1 gap-x-6 gap-y-10 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-6 xl:gap-x-8">
            <For each=move || books.clone() key=|b| b.id
            view=move |cx, b: BookUI| {
                view! { cx,
                <A href=format!("/book/{}", b.id) class="group">
                    <div class="aspect-h-1 aspect-w-1 w-full overflow-hidden rounded-lg bg-gray-200 xl:aspect-h-8 xl:aspect-w-7">
                    <img loading="lazy" referrerpolicy="no-referrer" src={b.thumbnail} class="h-full w-full object-contain object-center group-hover:opacity-75" />
                    </div>
                    <h3 class="mt-4 text-sm font-medium text-gray-900 overflow-hidden">{b.title}</h3>
                    <p class="mt-1 text-sm text-gray-500">{b.authors.join(", ")}</p>
                </A>}
            }/>
        </div>
        </div>
    }
}
