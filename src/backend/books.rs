use leptos::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = FastStorageBook::register();
}
#[server(FastStorageBook, "/api")]
pub async fn fast_storage_book(isbn: String) -> Result<(), ServerFnError> {
    let _ = crate::backend::db::get_client()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
