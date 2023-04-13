use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = Login::register();
    let _ = GetAccount::register();
}
#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    let ident = crate::backend::ldap::from_scope(cx)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let r = ident
        .bind(&username, &password)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    crate::backend::auth::try_add_new_account(cx, &r.uid, &r.display_name)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    crate::backend::auth::set_account_info(cx, &r.uid)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    leptos_axum::redirect(cx, "/assets-mgr");
    return Ok(());
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub uid: String,
    pub display_name: String,
    pub role: Role,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[server(GetAccount, "/api")]
pub async fn get_account(cx: Scope) -> Result<Option<UserSession>, ServerFnError> {
    let a = crate::backend::auth::account_info_from_cookies(cx).await;
    tracing::debug!("account info: {:?}", &a);

    Ok(a.map(|a| UserSession {
        uid: a.id,
        display_name: a.display_name,
        role: match a.role {
            crate::backend::auth::Role::Admin => Role::Admin,
            crate::backend::auth::Role::User => Role::User,
        },
    }))
}
