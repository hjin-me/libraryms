use leptos::ServerFnError::ServerError;
use leptos::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = FastStorageBook::register();
}
#[server(FastStorageBook, "/api")]
pub async fn fast_storage_book(isbn: String) -> Result<(), ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    bms.storage(isbn.as_str(), "some one")
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}
