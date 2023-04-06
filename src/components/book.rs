use crate::entity::Book;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BookState {
    Available,
    Borrowed,
    Returned,
    Lost,
    Deleted,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookAct {
    pub btn_type: String,
    pub method: String,
    pub path: String,
    pub text: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookUI {
    pub id: i64,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    // pub import_at: time::OffsetDateTime,
    // pub state: BookState,
    pub operator: String,
    pub operator_name: String,
    // pub operate_at: time::OffsetDateTime,
    pub thumbnail: String,
    pub actions: Vec<BookAct>,
}

// fn book_with_actions(book: &Book, current_user: &Option<Entity>) -> Vec<BookAct> {
//     let act_borrow = BookAction {
//         btn_type: "btn-primary".to_string(),
//         method: "post".to_string(),
//         path: format!("/book/borrow/{}", &book.id),
//         text: "借阅".to_string(),
//     };
//     let act_return = BookAction {
//         btn_type: "btn-success".to_string(),
//         method: "post".to_string(),
//         path: format!("/book/return/{}", &book.id),
//         text: "归还".to_string(),
//     };
//     let act_confirm = BookAction {
//         btn_type: "btn-info".to_string(),
//         method: "post".to_string(),
//         path: format!("/book/confirm/{}", &book.id),
//         text: "确认已归还".to_string(),
//     };
//     let act_lost = BookAction {
//         btn_type: "btn-dark".to_string(),
//         method: "post".to_string(),
//         path: format!("/book/lost/{}", &book.id),
//         text: "标记遗失".to_string(),
//     };
//     let act_reset = BookAction {
//         btn_type: "btn-secondary".to_string(),
//         method: "put".to_string(),
//         path: format!("/book/{}", &book.id),
//         text: "重置状态".to_string(),
//     };
//     let act_delete = BookAction {
//         btn_type: "btn-danger".to_string(),
//         method: "delete".to_string(),
//         path: format!("/book/{}", &book.id),
//         text: "删除".to_string(),
//     };
//     let current_uid = current_user
//         .as_ref()
//         .map(|u| u.uid.clone())
//         .unwrap_or("".to_string());
//     let role = current_user
//         .as_ref()
//         .map(|u| u.role.clone())
//         .unwrap_or(Role::User);
//     let mut actions = vec![];
//     if book.state == BookState::Available && current_uid != "" {
//         actions.push(act_borrow);
//     }
//     if book.state == BookState::Borrowed && current_uid == book.operator {
//         actions.push(act_return);
//     }
//     if book.state == BookState::Returned && role == Role::Admin {
//         actions.push(act_confirm);
//     }
//     if book.state == BookState::Lost && role == Role::Admin {
//         actions.push(act_reset);
//     }
//     if role == Role::Admin {
//         actions.push(act_lost);
//         actions.push(act_delete);
//     }
//     actions
// }

#[allow(non_snake_case)]
#[component]
pub fn BookDetailPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let book_id_fn = move || {
        params
            .with(|p| p.get("id").cloned().map(|i| i.parse::<i64>().unwrap()))
            .unwrap()
    };

    let b = create_resource(cx, book_id_fn, |id| async move {
        crate::api::books::book_detail(id).await
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
pub fn BookDetail(cx: Scope, #[prop()] book: Book) -> impl IntoView {
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
                            <button type="submit"
                                class="block rounded bg-green-600 px-5 py-2 text-sm font-medium text-white hover:bg-green-500"
                            >
                            "借阅"
                            </button>
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
             <div class="col-auto">
                 <label for="isbn" class="visually-hidden">"ISBN"</label>
                 <input type="text" placeholder="ISBN编号" class="form-control" id="isbn" name="isbn" value="" />
             </div>
             <div class="col-auto">
                 <button type="submit" class="btn btn-primary mb-3">"入库"</button>
             </div>
        </ActionForm>

    }
}

#[allow(non_snake_case)]
#[component]
pub fn BookGallery(cx: Scope) -> impl IntoView {
    let posts = create_resource(
        cx,
        || (),
        |_| async { crate::api::books::book_list(0, 10).await },
    );

    let g = move || match posts.read(cx) {
        None => None,
        Some(Err(_)) => None,
        Some(Ok(books)) => Some(view! {
            cx,
            <For
            each=move || books.clone()
            key=|b| b.id
            view=move |cx, b: Book| {
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
    let posts = create_resource(
        cx,
        || (),
        |_| async { crate::api::books::book_list(0, 10).await },
    );
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
             <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
                {move || match posts.read(cx) {
                    None => None,
                    Some(Err(_)) => None,
                    Some(Ok(books)) => {
                        Some(view! { cx,
                            <For
                            each=move || books.clone()
                            key=|b| b.id
                            view=move |cx, b: Book| {
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
                                }
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
