use crate::app::auth::{Entity, IdentOptional, IdentRequire};
use crate::app::common::filters;
use crate::app::AppState;
use crate::data::books::{Book, BookMS, BookState};
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
    bms.storage(&p.isbn, &u.uid).await.expect("图书入库失败");
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u))
            .await
            .expect("生成图书列表失败"),
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
pub struct ResetParams {
    book_id: i64,
}
pub async fn reset_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<ResetParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.reset(&p.book_id, &u.uid).await.unwrap();
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
    bms.confirm(&p.book_id, &u.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: handle_book_table(&bms, &0, &1000, &Some(u)).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}

#[derive(Deserialize, Serialize)]
pub struct LostParams {
    book_id: i64,
}
pub async fn lost_book(
    IdentRequire(u): IdentRequire,
    State(s): State<AppState>,
    Path(p): Path<LostParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.lost(&p.book_id, &u.uid).await.unwrap();
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
        .map(|b| book_with_actions(b, current_user))
        .collect())
}
fn book_with_actions(book: &Book, current_user: &Option<Entity>) -> BookUI {
    let act_borrow = BookAction {
        btn_type: "btn-primary".to_string(),
        method: "post".to_string(),
        path: format!("/book/borrow/{}", &book.id),
        text: "借阅".to_string(),
    };
    let act_return = BookAction {
        btn_type: "btn-success".to_string(),
        method: "post".to_string(),
        path: format!("/book/return/{}", &book.id),
        text: "归还".to_string(),
    };
    let act_confirm = BookAction {
        btn_type: "btn-info".to_string(),
        method: "post".to_string(),
        path: format!("/book/confirm/{}", &book.id),
        text: "确认已归还".to_string(),
    };
    let act_lost = BookAction {
        btn_type: "btn-dark".to_string(),
        method: "post".to_string(),
        path: format!("/book/lost/{}", &book.id),
        text: "标记遗失".to_string(),
    };
    let act_reset = BookAction {
        btn_type: "btn-secondary".to_string(),
        method: "put".to_string(),
        path: format!("/book/{}", &book.id),
        text: "重置状态".to_string(),
    };
    let act_delete = BookAction {
        btn_type: "btn-danger".to_string(),
        method: "delete".to_string(),
        path: format!("/book/{}", &book.id),
        text: "删除".to_string(),
    };

    let mut actions = vec![];
    match book.state {
        BookState::Available => {
            actions.push(act_borrow);
        }
        BookState::Borrowed => {
            if let Some(u) = current_user {
                if u.uid == book.operator {
                    actions.push(act_return);
                }
            }
        }
        BookState::Returned => {
            actions.push(act_confirm);
        }
        BookState::Unknown => {
            actions.push(act_reset);
        }
        BookState::Lost => {
            actions.push(act_reset);
        }
        _ => {
            actions.push(act_reset);
        }
    }
    actions.push(act_lost);
    actions.push(act_delete);
    BookUI {
        book: book.clone(),
        actions,
    }
}

#[derive(Template)]
#[template(path = "book_detail.html")]
struct BookDetailTemplate {
    current_user: Option<Entity>,
    item: BookUI,
}
#[derive(Deserialize, Serialize)]
pub struct BookDetailParams {
    book_id: i64,
}
pub async fn book_detail(
    IdentOptional(entity): IdentOptional,
    Path(p): Path<BookDetailParams>,
    State(s): State<AppState>,
) -> impl IntoResponse {
    let b = s.book_ms.get_one_by_id(&p.book_id).await.unwrap();

    let template = BookDetailTemplate {
        current_user: None,
        item: book_with_actions(&b, &entity),
    };
    crate::app::common::HtmlTemplate(template)
}
