use leptos::*;
#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = Login::register();
}
#[server(Login, "/api")]
pub async fn login(cx: Scope) -> Result<(), ServerFnError> {
    leptos_axum::redirect(cx, "/assets-mgr");
    Ok(())
}
