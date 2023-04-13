use crate::components::assets::*;
use crate::components::auth::*;
use crate::components::book::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::SsrMode::InOrder;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn BlogApp(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let formatter = |text| format!("{text} — 图书管理系统 - 安天移动安全");

    let login_action = create_server_action::<crate::api::auth::Login>(cx);
    view! {
      cx,
      <Html lang="zh-hans"/>
      <Meta charset="utf-8"/>
          <Title
        // reactively sets document.title when `name` changes
        text="首页"
        // applies the `formatter` function to the `text` value
        formatter=formatter
      />
      <Stylesheet href="/pkg/libraryms.css"/>

      <Router>
        <Header action=login_action />
        <main>
        <Routes>
        <Route path="" view=|cx| view! {cx,<DefaultPage/>} ssr=InOrder/>
        <Route path="book/:id" view=|cx| view! {cx,<BookDetailPage/>} ssr=InOrder/>
        <Route path="assets-mgr" view=|cx| view! {cx,<AssetsPage/>} ssr=InOrder/>
        <Route path="login" view= move |cx| view! {cx,<LoginPage action=login_action/>} ssr=InOrder />
        <Route path="my" view= move |cx| view! {cx,<div>"My"</div>} ssr=InOrder />
        </Routes>
        </main>
      </Router>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn Header(
    cx: Scope,
    action: Action<crate::api::auth::Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map(cx);
    // search stored as ?q=
    let search = move || query().get("q").cloned().unwrap_or_default();

    // let account = create_resource(cx, || {}, move async |_| { get_account(cx).await });
    let account = create_resource(
        cx,
        move || (action.version().get()),
        move |_| crate::api::auth::get_account(cx),
    );

    let u = {
        move || {
            account.read(cx).map(|user| match user {
            Err(e) => view! {cx,
                            <span>{format!("Login error: {}", e.to_string())}</span>
                        }.into_view(cx),
            Ok(None) => view! {cx,
                            <A
                                href="/login" class="rounded-lg bg-blue-600 px-5 py-2 text-sm font-medium text-white">
                                "登录"
                            </A>
                        }.into_view(cx),
            Ok(Some(user)) => view! {cx,
                            <span>{format!("欢迎爱学习的 {}", user.display_name)}</span>
                        }.into_view(cx)
        })
        }
    };

    view! {
        cx,
      <header aria-label="Site Header" class="shadow-sm">
        <div class="mx-auto flex h-16 max-w-screen-xl items-center justify-between px-4">
          <div class="flex w-0 flex-1 lg:hidden">
            <button class="rounded-full bg-gray-100 p-2 text-gray-600" type="button">
             <span class="sr-only">"Account"</span>
              <svg
            class="h-5 w-5"
            fill="none"
            stroke="currentColor"
            viewbox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
                >
            <path
              d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
            ></path>
              </svg>
            </button>
          </div>

        <div class="flex items-center gap-4">
        <Form class="mb-0 hidden lg:flex" method="GET" action="">
          <div class="relative">
            <input
                class="h-10 rounded-lg border-gray-200 pr-10 text-sm placeholder-gray-300 focus:z-10"
                placeholder="Search..."
                type="search"
                name="q"
                value=search
            />

            <button
              type="submit"
              class="absolute inset-y-0 right-0 rounded-r-lg p-2 text-gray-600"
            >
              <span class="sr-only">"Submit Search"</span>
              <svg
                class="h-5 w-5"
                fill="currentColor"
                viewbox="0 0 20 20"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  clip-rule="evenodd"
                  d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                  fill-rule="evenodd"
                ></path>
              </svg>
            </button>
          </div>
        </Form>
      </div>


        <div class="flex w-0 flex-1 justify-end lg:hidden">
        <button class="rounded-full bg-gray-100 p-2 text-gray-500" type="button">
          <span class="sr-only">"Menu"</span>
          <svg
            class="h-5 w-5"
            fill="currentColor"
            viewbox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              clip-rule="evenodd"
              d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
              fill-rule="evenodd"
            ></path>
          </svg>
        </button>
      </div>

        <nav
        aria-label="Site Nav"
        class="hidden items-center justify-center gap-8 text-sm font-medium lg:flex lg:w-0 lg:flex-1"
      >
        <A class="text-gray-900" href="/">"图书馆"</A>
        <A class="text-gray-900" href="/my">"我的借阅"</A>
        <A class="text-gray-900" href="/assets-mgr">"资产管理"</A>
      </nav>

        <div class="hidden items-center gap-4 lg:flex">
        <Suspense fallback= move || view! {cx, <span>"Loading"</span>}.into_any()>
            {u}
        </Suspense>
      </div>

        </div>
        <div class="border-t border-gray-100 lg:hidden">
      <nav
        class="flex items-center justify-center overflow-x-auto p-4 text-sm font-medium"
      >
        <A class="flex-shrink-0 pl-4 text-gray-900" href="/">"图书馆"</A>
        <A class="flex-shrink-0 pl-4 text-gray-900" href="/my">"我的借阅"</A>
        <A class="flex-shrink-0 pl-4 text-gray-900" href="/assets-mgr">"资产管理"</A>
      </nav>
    </div>
        </header>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn DefaultPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <BookGallery />
    }
}
