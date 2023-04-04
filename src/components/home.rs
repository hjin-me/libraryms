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
        <Stylesheet href="https://unpkg.com/bootstrap@5.2.3/dist/css/bootstrap.min.css"/>
        <Router>
            <Header />
    <div class="container-fluid">
        <div class="row">
            <div class="col-12">
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
            </div>
        </div>
    </div>
        </Router>
            }
}
#[allow(non_snake_case)]
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
                cx,
            <div class="container-fluid">
        <header
            class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-4 border-bottom">
            <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none"></a>

            <ul class="nav col-12 col-md-auto mb-2 justify-content-center mb-md-0">
                <li><a href="/books" class="nav-link px-2 link-secondary">"图书管理"</a></li>
            </ul>

            <div class="col-md-3 text-end">
                <a href="/authentication" class="btn btn-outline-primary me-2">"登陆"</a>
            </div>
        </header>
    </div>
        }
}
#[allow(non_snake_case)]
#[component]
pub fn DefaultPage(cx: Scope) -> impl IntoView {
    view! {
                cx,
            <div class="container-fluid">

        <BookStorage/>

        <div class="row">
            <div class="col-12">
            </div>
        </div>
    </div>
        }
}
