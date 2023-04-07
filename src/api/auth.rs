use leptos::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = Login::register();
}
#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    // use axum::body::HttpBody;
    // dbg!(username, password);
    let ident = crate::backend::ldap::get_ldap_ident()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let r = ident
        .bind(&username, &password)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    if r {
        crate::backend::auth::set_account_info(cx, &username);
        leptos_axum::redirect(cx, "/assets-mgr");
        return Ok(());
    }

    Ok(())
}
