use crate::entity::{Book, BookState};
use leptos::ServerFnError::ServerError;
use leptos::*;
use serde::{Deserialize, Serialize};
use tracing::trace;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = FastStorageBook::register();
    let _ = BookList::register();
    let _ = BookDetail::register();
}
#[server(FastStorageBook, "/api")]
pub async fn fast_storage_book(isbn: String) -> Result<(), ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    bms.storage(isbn.as_str(), "songsong")
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}

#[server(BookList, "/api")]
pub async fn book_list(offset: i64, limit: i64) -> Result<Vec<BookUI>, ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let books = bms
        .list(&limit, &offset)
        .await
        .map_err(|e| ServerError(e.to_string()))?
        .iter()
        .map(|b| BookUI::from(b))
        .collect();

    Ok(books)
}

#[server(BookDetail, "/api")]
pub async fn book_detail(id: i64) -> Result<BookUI, ServerFnError> {
    trace!("id: {}", id);
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let book = bms
        .get_one_by_id(&id)
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(book.into())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BookUI {
    pub id: i64,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub import_at: time::OffsetDateTime,
    pub state: BookState,
    pub operator: String,
    pub operator_name: String,
    pub operate_at: time::OffsetDateTime,
    pub thumbnail: String,
    actions: Vec<BookAction>,
}

#[cfg(feature = "ssr")]
impl From<Book> for BookUI {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            isbn: value.isbn,
            title: value.title,
            authors: value.authors,
            publisher: value.publisher,
            import_at: value.import_at,
            state: value.state,
            operator: value.operator,
            operator_name: value.operator_name,
            operate_at: value.operate_at,
            thumbnail: value.thumbnail,
            actions: vec![],
        }
    }
}
#[cfg(feature = "ssr")]
impl From<&Book> for BookUI {
    fn from(value: &Book) -> Self {
        Self::from(value.clone())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BookAction {
    btn_type: String,
    text: String,
}
