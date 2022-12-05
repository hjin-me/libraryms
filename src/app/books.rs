use crate::app::auth::Entity;
use crate::app::AppState;
use crate::data::books::Book;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;

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
    crate::app::HtmlTemplate(template)
}
