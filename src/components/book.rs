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
    let b = BookUI {
        id: 1,
        isbn: "123".to_string(),
        title: "123".to_string(),
        authors: vec!["123".to_string()],
        publisher: "123".to_string(),
        // import_at: time::OffsetDateTime::now_utc(),
        // state: BookState::Available,
        operator: "123".to_string(),
        operator_name: "123".to_string(),
        // operate_at: time::OffsetDateTime::now_utc(),
        thumbnail: "123".to_string(),
        actions: vec![],
    };

    view! {
        cx,
        <BookDetail book=b/>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn BookDetail(cx: Scope, #[prop()] book: BookUI) -> impl IntoView {
    let book_acts: Vec<_> = vec![
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "借阅".to_string(),
        },
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "归还".to_string(),
        },
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "确认".to_string(),
        },
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "重置状态".to_string(),
        },
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "丢失".to_string(),
        },
        BookAct {
            btn_type: "btn-primary".to_string(),
            method: "get".to_string(),
            path: "/book/1".to_string(),
            text: "删除".to_string(),
        },
    ].iter().map(|act| {
        let act = act.clone();
        view! { cx,  <button type="button" class=format!("btn {} btn-sm", act.btn_type)>{act.text}</button> }
    }).collect();

    view! {
                cx,
    <div class="container">
        <div class="row">
            <div class="col-3">
                <img src=book.thumbnail rel="noreferrer" referrerpolicy="no-referrer" class="img-thumbnail"
                     alt="没找到缩略图"/>
            </div>
            <div class="col-9">
                <h3>{book.title}</h3>
                <p>"作者："{book.authors.join(", ")} </p>
                <p>"ISBN: "{book.isbn}</p>
                <p>"出版商："{book.publisher}</p>

                <p>
                    {book_acts}
                </p>
            </div>
        </div>
    </div>
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
