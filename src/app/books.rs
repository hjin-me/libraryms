use crate::app::auth::{Entity, IdentOptional, IdentRequire};
use crate::app::common::filters;
use crate::app::AppState;
use crate::data::books::{Book, BookMS};
use askama::Template;
use axum::extract::{Form, Path, State};
use axum::response::IntoResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "book_list.html")]
struct BookListTemplate {
    current_user: Option<Entity>,
    books: Vec<BookUI>,
}

pub async fn book_list_get(
    IdentOptional(entity): IdentOptional,
    State(s): State<AppState>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    let template = BookListTemplate {
        current_user: entity.clone(),
        books: handle_book_table(&bms, &0, &1000, &entity).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}
#[derive(Deserialize, Serialize)]
pub struct SimpleStorageParams {
    isbn: String,
}

pub async fn simple_storage(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Form(p): Form<SimpleStorageParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.storage(&p.isbn, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}
#[derive(Deserialize, Serialize)]
pub struct DeleteParams {
    book_id: i64,
}
pub async fn delete_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<DeleteParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.delete(&p.book_id, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}
#[derive(Deserialize, Serialize)]
pub struct BorrowParams {
    book_id: i64,
}
pub async fn borrow_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<BorrowParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.borrow(&p.book_id, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}
#[derive(Deserialize, Serialize)]
pub struct ReturnParams {
    book_id: i64,
}
pub async fn return_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<ReturnParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.revert_to(&p.book_id, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}

#[derive(Deserialize, Serialize)]
pub struct ConfirmParams {
    book_id: i64,
}
pub async fn confirm_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<ConfirmParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.revert_to(&p.book_id, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "books_table.html")]
struct BooksTableTemplate {
    books: Vec<BookUI>,
}

struct BookAction {
    btn_type: String,
    method: String, // post/get/delete/put
    path: String,   // /book/{{book.id}}
    text: String,
}
struct BookUI {
    book: Book,
    actions: Vec<BookAction>,
}

async fn handle_book_table(
    bms: &BookMS,
    offset: &i64,
    limit: &i64,
    current_user: &Option<Entity>,
) -> Result<Vec<BookUI>, Box<dyn std::error::Error>> {
    Ok(bms
        .list(&limit, &offset)
        .await
        .unwrap()
        .iter()
        .map(|b| BookUI {
            book: b.clone(),
            actions: vec![
                BookAction {
                    btn_type: "btn-primary".to_string(),
                    method: "post".to_string(),
                    path: format!("/book/borrow/{}", &b.id),
                    text: "借阅".to_string(),
                },
                BookAction {
                    btn_type: "btn-success".to_string(),
                    method: "post".to_string(),
                    path: format!("/book/return/{}", &b.id),
                    text: "归还".to_string(),
                },
                BookAction {
                    btn_type: "btn-info".to_string(),
                    method: "post".to_string(),
                    path: format!("/book/confirm/{}", &b.id),
                    text: "确认已归还".to_string(),
                },
                BookAction {
                    btn_type: "btn-dark".to_string(),
                    method: "post".to_string(),
                    path: format!("/book/lost/{}", &b.id),
                    text: "标记遗失".to_string(),
                },
                BookAction {
                    btn_type: "btn-secondary".to_string(),
                    method: "put".to_string(),
                    path: format!("/book/{}", &b.id),
                    text: "重置状态".to_string(),
                },
                BookAction {
                    btn_type: "btn-danger".to_string(),
                    method: "delete".to_string(),
                    path: format!("/book/{}", &b.id),
                    text: "删除".to_string(),
                },
            ],
        })
        .collect())
}
