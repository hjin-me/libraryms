use crate::components::book::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn BlogApp(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    let formatter = |text| format!("{text} — 图书管理系统 - 安天移动安全");

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
        <Stylesheet href="/pkg/hj.css"/>
        <Router>
            <Header />
            <main>
                <Routes>
                       <Route path="" view=|cx| view! {
                        cx,
                        <DefaultPage/>
                } ssr=SsrMode::InOrder/> //Route
                        <Route path="book/:id" view=|cx| view! {
                        cx,
                        <BookDetailPage/>
                } ssr=SsrMode::InOrder/> //Route
                </Routes>
            </main>
        </Router>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
                    cx,
    <header class="bg-white">
      <nav class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8" aria-label="Global">
        <div class="flex lg:flex-1">
          <a href="/" class="-m-1.5 p-1.5">
            <span class="sr-only">"Your Company"</span>
            <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600" alt=""/>
          </a>
        </div>
        <div class="flex lg:hidden">
          <button type="button" class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700">
            <span class="sr-only">"Open main menu"</span>
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
          </button>
        </div>
        <div class="hidden lg:flex lg:gap-x-12">
            <A href="/" class="text-sm font-semibold leading-6 text-gray-900">"图书馆"</A>
            <A href="/my" class="text-sm font-semibold leading-6 text-gray-900">"我的借阅"</A>
            <A href="/assets-mgr" class="text-sm font-semibold leading-6 text-gray-900">"资产管理"</A>
        </div>

        <div class="hidden lg:flex lg:flex-1 lg:justify-end">
          <a href="#" class="text-sm font-semibold leading-6 text-gray-900">"Log in "<span aria-hidden="true">"→"</span></a>
        </div>
      </nav>
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
