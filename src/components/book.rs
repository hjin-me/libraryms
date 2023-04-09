use crate::api::books::{BookAction, BookUI};
use leptos::*;
use leptos_router::*;
use time::OffsetDateTime;

#[allow(non_snake_case)]
#[component]
pub fn BookDetailPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let book_id_fn = move || {
        params
            .with(|p| p.get("id").cloned().map(|i| i.parse::<i64>().unwrap()))
            .unwrap()
    };

    let b = create_resource(cx, book_id_fn, move |id| {
        crate::api::books::book_detail(cx, id)
    });

    let g = move || match b.read(cx) {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(book)) => Some(view! {
            cx,
            <BookDetail book=book/>
        }),
    };

    view! {
        cx,
        <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
            {g}
        </Suspense>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BookDetail(cx: Scope, #[prop()] book: BookUI) -> impl IntoView {
    let act_btn :Vec<_>= book.actions.iter().map(move |a| {
        match a {
            BookAction::Borrow => view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "借阅"
                    </button>
            },
            BookAction::Lost => view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "标记遗失"
                    </button>
            },
            BookAction::Return => view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "归还"
                    </button>
            },
            BookAction::Reset => view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "重置状态"
                    </button>
            },
            BookAction::Delete=> view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "删除"
                    </button>
            },
            BookAction::Lost => view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "标记遗失"
                    </button>
            },
            _ => {

                view! {
                    cx,
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "其他"
                    </button>
                }
            }
        }
    }).collect();
    view! {
    cx,
    <section>
        <div class="relative mx-auto max-w-screen-xl px-4 py-8">
            <div class="grid grid-cols-3 items-start gap-8">
                <div class="col-span-1">
                    <img loading="lazy" referrerpolicy="no-referrer" src=book.thumbnail class="aspect-square w-full rounded-xl object-cover" />
                </div>
                <div class="sticky top-0 col-span-2">
                    <strong class="rounded-full border border-blue-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-blue-600">
                        "可借阅"
                    </strong>
                    <div class="mt-8 flex justify-between">
                        <div class="max-w-[50ch] space-y-2">
                            <h1 class="text-xl font-bold sm:text-2xl">{book.title}</h1>
                            <p class="text-sm">{book.authors.join(", ")}</p>
                            <p class="text-sm">{book.isbn}</p>
                            <p class="text-sm">{book.publisher}</p>
                            {act_btn}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BookStorage(cx: Scope) -> impl IntoView {
    let fast_storage_book_act = create_server_action::<crate::api::books::FastStorageBook>(cx);
    view! {
        cx,
        <ActionForm class="row g-3" action=fast_storage_book_act>
            <label for="isbn" class="block overflow-hidden rounded-md border border-gray-200 px-3 py-2 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600">
                <span class="text-xs font-medium text-gray-700"> "ISBN" </span>
                <input type="text" id="isbn" name="isbn" class="mt-1 w-full border-none p-0 focus:border-transparent focus:outline-none focus:ring-0 sm:text-sm"/>
            </label>

            <button type="submit" class="inline-block shrink-0 rounded-md border border-blue-600 bg-blue-600 px-12 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500">"入库"</button>

        </ActionForm>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BookGallery(cx: Scope) -> impl IntoView {
    let posts = create_resource(cx, || (), move |_| crate::api::books::book_list(cx, 0, 10));

    let g = move || match posts.read(cx) {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(books)) => Some(view! {
            cx,
            <For
            each=move || books.clone()
            key=|b| b.id
            view=move |cx, b: BookUI| {
                view! {
                    cx,
              <A href=format!("/book/{}", b.id) class="group">
          <div class="aspect-h-1 aspect-w-1 w-full overflow-hidden rounded-lg bg-gray-200 xl:aspect-h-8 xl:aspect-w-7">
            <img loading="lazy" referrerpolicy="no-referrer" src={b.thumbnail} class="h-full w-full object-contain object-center group-hover:opacity-75" />
          </div>
          <h3 class="mt-4 text-sm font-medium text-gray-900 overflow-hidden">{b.title}</h3>
          <p class="mt-1 text-sm text-gray-500">{b.authors.join(", ")}</p>
        </A>
                }
            }/>
        }),
    };
    view! {
          cx,
          <div class="mx-auto max-w-2xl px-4 sm:px-6 lg:max-w-7xl lg:px-8">
      <h2 class="sr-only">"Products"</h2>

      <div class="grid grid-cols-1 gap-x-6 gap-y-10 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-6 xl:gap-x-8">
         <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
            {g}
        </Suspense>
      </div>
    </div>
      }
}

#[allow(non_snake_case)]
#[component]
pub fn BookList(cx: Scope) -> impl IntoView {
    let posts = create_resource(cx, || (), move |_| crate::api::books::book_list(cx, 0, 10));
    view! {
                cx,
            <div class="overflow-x-auto">
      <table class="min-w-full divide-y-2 divide-gray-200 text-sm">
        <thead>
          <tr>
            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
              "#"
            </th>
            <th
              class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"
            >
              "书名"
            </th>
            <th
              class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"
            >
              "作者"
            </th>
            <th
              class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"
            >
              "上次变更"
            </th> <th
              class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"
            >
              "ISBN"
            </th> <th
              class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"
            >
              "出版商"
            </th>
            <th class="px-4 py-2"></th>
          </tr>
        </thead>

        <tbody class="divide-y divide-gray-200">
          <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }.into_any()>
                {move || match posts.read(cx) {
                    None => None,
                    Some(Err(_)) => None,
                    Some(Ok(books)) => {
                        Some(view! { cx,
                            <For
                            each=move || books.clone()
                            key=|b| b.id
                            view=move |cx, b: BookUI| {
                                view! { cx,
            <tr>
            <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{b.id}</th>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700"><span class="ellipsis">{b.title}</span></td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.authors.join(", ")}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.state.to_string()}</td>

            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.operate_at.to_string()}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.isbn}</td>
            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.publisher}</td>
                                    <td class="whitespace-nowrap px-4 py-2">
              <A
                href="#"
                class="inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700"
              >
                "View"
              </A>
            </td>
                    </tr>
                                }.into_any()
                            }
                            />
                        })
                    }
                }}
            </Suspense>
        </tbody>
      </table>
    </div>
    }
}

pub fn from_now(date: OffsetDateTime) -> String {
    use std::ops::Sub;
    let d = date.sub(OffsetDateTime::now_utc());
    let append: &str = if d.is_positive() { "后" } else { "前" };
    if d.whole_seconds().abs() < 60 {
        return "刚才".to_string();
    }
    if d.whole_minutes().abs() < 60 {
        return format!("{} 分钟{}", d.whole_minutes().abs(), append);
    }
    if d.whole_hours().abs() < 24 {
        return format!("{} 小时{}", d.whole_hours().abs(), append);
    }
    if d.whole_days().abs() < 30 {
        return format!("{} 天{}", d.whole_days().abs(), append);
    }
    let out_format = time::format_description::parse(
        "[year]-[month]-[day] [hour padding:none]:[minute]:[second]",
    )
    .unwrap();
    date.format(&out_format).unwrap()
}
