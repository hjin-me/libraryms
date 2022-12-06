use crate::app::auth::{Entity, IdentRequire};
use crate::app::common::filters;
use crate::app::AppState;
use crate::data::books::Book;
use askama::Template;
use axum::extract::{Form, State};
use axum::response::IntoResponse;
use serde_derive::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "book_list.html")]
struct BookListTemplate {
    current_user: Option<Entity>,
    books: Vec<Book>,
}

pub async fn book_list_get(State(s): State<AppState>) -> impl IntoResponse {
    let bms = s.book_ms;
    let bs = bms.list(&100, &0).await.unwrap();
    let template = BookListTemplate {
        current_user: None,
        books: bs,
    };
    crate::app::common::HtmlTemplate(template)
}
#[derive(Deserialize, Serialize)]
pub struct SimpleStorageParams {
    isbn: String,
}
#[derive(Template)]
#[template(path = "books_table.html")]
struct BooksTableTemplate {
    books: Vec<Book>,
}
pub async fn simple_storage(
    IdentRequire(current_user): IdentRequire,
    State(s): State<AppState>,
    Form(p): Form<SimpleStorageParams>,
) -> impl IntoResponse {
    let bms = s.book_ms;
    bms.storage(&p.isbn, &current_user.uid).await.unwrap();
    let template = BooksTableTemplate {
        books: bms.list(&100, &0).await.unwrap(),
    };
    crate::app::common::HtmlTemplate(template)
}
