use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn Pagination(
    cx: Scope,
    #[prop(into)] pn: Signal<i64>,
    #[prop(into)] set_pn: WriteSignal<i64>,
) -> impl IntoView {
    view! {
            cx,
    <div class="inline-flex justify-center gap-1">
      <button
        type="button"
        class="inline-flex h-8 w-8 items-center justify-center rounded border border-gray-100"
        on:click=move |_| {
            set_pn.update(|pn| {
                if pn.clone() > 1 {
                    *pn -= 1
                }
            })
        }
      >
        <span class="sr-only">"Prev Page"</span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-3 w-3"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
            clip-rule="evenodd"
          />
        </svg>
      </button>

      <div>
        <label for="PaginationPage" class="sr-only">"Page"</label>

        <input
          type="number"
          class="h-8 w-12 rounded border border-gray-100 p-0 text-center text-xs font-medium [-moz-appearance:_textfield] [&::-webkit-outer-spin-button]:m-0 [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:m-0 [&::-webkit-inner-spin-button]:appearance-none"
          min="1"
          value=pn
          id="PaginationPage"
        />
      </div>

      <a
        href="#"
        class="inline-flex h-8 w-8 items-center justify-center rounded border border-gray-100"
        on:click=move |_| {
            set_pn.update(|pn| *pn += 1)
        }
      >
        <span class="sr-only">"Next Page"</span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-3 w-3"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
            clip-rule="evenodd"
          />
        </svg>
      </a>
    </div>

        }
}
