use crate::entity::Book;
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
pub async fn book_list(offset: i64, limit: i64) -> Result<Vec<Book>, ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let books = bms
        .list(&limit, &offset)
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(books)
}

#[server(BookDetail, "/api")]
pub async fn book_detail(id: i64) -> Result<Book, ServerFnError> {
    trace!("id: {}", id);
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let book = bms
        .get_one_by_id(&id)
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(book)
}
