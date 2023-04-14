use crate::api::books::{BookAction, BookUI};
use crate::api::entity::BookState;
use crate::components::pagination::*;
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

    let borrow_act = create_server_action::<crate::api::books::BorrowBook>(cx);
    let revert_to_act = create_server_action::<crate::api::books::ReturnBook>(cx);

    let b = create_resource(
        cx,
        move || {
            (
                book_id_fn(),
                borrow_act.version().get(),
                revert_to_act.version().get(),
            )
        },
        move |(id, _, _)| crate::api::books::book_detail(cx, id),
    );

    let g = move || match b.read(cx) {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(book)) => Some(view! {
            cx,
            <BookDetail book=book borrow=borrow_act revert=revert_to_act/>
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
pub fn BookDetail(
    cx: Scope,
    #[prop()] book: BookUI,
    borrow: Action<crate::api::books::BorrowBook, Result<(), ServerFnError>>,
    revert: Action<crate::api::books::ReturnBook, Result<(), ServerFnError>>,
) -> impl IntoView {
    let act_btn :Vec<_>= book.actions.iter().filter(|a| {
        match a {
            BookAction::Borrow => true,
            BookAction::Return => true,
            _ => false
        }
    }).map(move |a| {
        match a {
            BookAction::Borrow => view! {
                    cx,
                <ActionForm action=borrow>
                    <input type="hidden" name="id" value=book.id/>
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "借阅"
                    </button>
                </ActionForm>
            }.into_view(cx),
            BookAction::Return => view! {
                    cx,
                <ActionForm action=revert>
                    <input type="hidden" name="id" value=book.id/>
                    <button type="submit" class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500">
                    "归还"
                    </button>
                </ActionForm>
            }.into_view(cx),
            _ => view! { cx, <span></span> }.into_view(cx)
        }
    }).collect();

    let book_state = move || match book.state {
        BookState::Available => view! {cx,
        <strong class="rounded-full border border-green-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-green-600">
            "可借阅"
            </strong>},
        BookState::Borrowed => {
            view! {cx, <strong class="rounded-full border border-gray-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-gray-600">
            "已借出"
            </strong>}
        }
        BookState::Returned => {
            view! {cx, <strong class="rounded-full border border-yellow-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-yellow-600">
            "归还中"
            </strong>}
        }
        BookState::Lost => {
            view! {cx, <strong class="rounded-full border border-red-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-red-600">
            "已丢失"
            </strong>}
        }
        BookState::Deleted => {
            view! {cx, <strong class="rounded-full border border-orange-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-orange-600">
            "已损坏"
            </strong>}
        }
        BookState::Unknown => {
            view! {cx, <strong class="rounded-full border border-purple-600 bg-gray-100 px-3 py-0.5 text-xs font-medium tracking-wide text-purple-600">
            "状态异常"
            </strong>}
        }
    };

    view! {
    cx,
    <section>
        <div class="relative mx-auto max-w-screen-xl px-4 py-8">
            <div class="grid grid-cols-3 items-start gap-8">
                <div class="col-span-1">
                    <img loading="lazy" referrerpolicy="no-referrer" src=book.thumbnail class="aspect-square w-full rounded-xl object-cover" />
                </div>
                <div class="sticky top-0 col-span-2">

                    {book_state}
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
        <ActionForm class="grid grid-cols-2 gap-4  row g-3" action=fast_storage_book_act>
            <div>
                <label for="isbn" class="sr-only">"ISBN"</label>
                    <input type="text" name="isbn"
                        class="w-full rounded-lg border-gray-200 p-3 text-sm"
                        placeholder="输入书籍ISBN" autocomplete="off"/>
            </div>

            <button type="submit" class="inline-block shrink-0 rounded-md border border-blue-600 bg-blue-600 px-12 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500">"入库"</button>

        </ActionForm>
    }
}

// #[allow(non_snake_case)]
// #[component]
// pub fn BookGallery(cx: Scope) -> impl IntoView {
//     // reactive access to URL query strings
//     let query = use_query_map(cx);
//     // search stored as ?q=
//     let search = move || query().get("q").cloned();
//
//     let posts = create_resource(
//         cx,
//         move || (search()),
//         move |q| crate::api::books::book_list(cx, None, None, q),
//     );
//
//     let g = move || match posts.read(cx) {
//         None => None,
//         Some(Err(_)) => None,
//         Some(Ok(books)) => Some(view! {
//             cx,
//             <For
//             each=move || books.clone()
//             key=|b| b.id
//             view=move |cx, b: BookUI| {
//                 view! {
//                     cx,
//                 <A href=format!("/book/{}", b.id) class="group">
//                 <div class="aspect-h-1 aspect-w-1 w-full overflow-hidden rounded-lg bg-gray-200 xl:aspect-h-8 xl:aspect-w-7">
//                     <img loading="lazy" referrerpolicy="no-referrer" src={b.thumbnail} class="h-full w-full object-contain object-center group-hover:opacity-75" />
//                 </div>
//                 <h3 class="mt-4 text-sm font-medium text-gray-900 overflow-hidden">{b.title}</h3>
//                 <p class="mt-1 text-sm text-gray-500">{b.authors.join(", ")}</p>
//                 </A>
//                 }
//             }/>
//         }),
//     };
//     view! {
//           cx,
//           <div class="mx-auto max-w-2xl px-4 my-4 lg:max-w-7xl">
//       <h2 class="sr-only">"Products"</h2>
//
//       <div class="grid grid-cols-1 gap-x-6 gap-y-10 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-6 xl:gap-x-8">
//          <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
//             {g}
//         </Suspense>
//       </div>
//     </div>
//       }
// }

#[allow(non_snake_case)]
#[component]
pub fn BookList(cx: Scope) -> impl IntoView {
    let confirm_act = create_server_action::<crate::api::books::ConfirmReturnBook>(cx);

    let (pn, set_pn) = create_signal(cx, 1);
    let posts = create_resource(
        cx,
        move || (pn.get(), confirm_act.version().get()),
        move |(pn, _)| {
            let offset = (pn - 1) * 10;
            crate::api::books::book_list(cx, Some(offset), None, None)
        },
    );
    view! {
        cx,
        <div>
            <div class="my-4 px-4">
                <Pagination pn=pn set_pn=set_pn/>
            </div>
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
                      "状态"
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
                Some(view! {
                    cx,
                    <For
                    each=move || books.clone()
                    key=|b| b.id
                    view=move |cx, b: BookUI| {
                        view! { cx,
                            <tr>
                            <th class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{b.id}</th>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700 " title=&b.title>
                                <div class="truncate max-w-xs">{b.title}</div>
                                <div class="truncate max-w-xs pl-4">{b.authors.join(", ")}</div>
                            </td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                            {b.state.to_string()}
                            {
                                move || if b.state == BookState::Returned {
                                Some(view! {
                                    cx,
                                    <ActionForm action=confirm_act class="inline-block">
                                    <input type="hidden" value=b.id name="id" />
                                    <button type="submit"
                                    class="ml-4 inline-block rounded bg-green-600 px-4 py-2 text-xs font-medium text-white hover:bg-green-700"
                                    >
                                    "确认归还"
                                    </button></ActionForm>
                                })
                            } else {
                                None
                            }
                            }
                            </td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.isbn}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{b.publisher}</td>
                            <td class="whitespace-nowrap px-4 py-2">
                            {
                                move || b.actions.iter().filter(|a| {
                                match a {
                                    BookAction::Lost => true,
                                    BookAction::Reset => true,
                                    BookAction::Delete => true,
                                    _ => false,
                                }
                            }).map(|a| match a {
                                BookAction::Lost => view! {
                                    cx,
                                    <A
                                    href=format!("/book/{}/lost", b.id)
                                    class="mr-4 inline-block rounded bg-black px-4 py-2 text-xs font-medium text-white"
                                    >
                                    "丢失"
                                    </A>
                                }.into_view(cx),
                                BookAction::Reset => view! {
                                    cx,
                                    <A
                                    href=format!("/book/{}/reset", b.id)
                                    class="mr-4 inline-block rounded bg-yellow-600 px-4 py-2 text-xs font-medium text-white hover:bg-yellow-700"
                                    >
                                    "重置"
                                    </A>
                                }.into_view(cx),
                                BookAction::Delete => view! {
                                    cx,
                                    <A
                                    href=format!("/book/{}/delete", b.id)
                                    class="mr-4 inline-block rounded bg-red-600 px-4 py-2 text-xs font-medium text-white hover:bg-red-700"
                                    >
                                    "删除"
                                    </A>
                                }.into_view(cx),
                                _ => view! { cx, <div/>}.into_view(cx)
                            }).collect::<Vec<_>>()
                            }
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
        </div>
    }
}

// <Pagination
// current=move || page
// total=move || total
// page_size=move || page_size
// on_change=move |p| {
// page = p;
// posts.update(cx);
// }/>
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
