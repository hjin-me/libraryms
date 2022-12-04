use crate::app::auth::{gen_access_token, gen_exchange_token};
use crate::app::{common, AppState};
use askama::Template;
use axum::extract::{Query, State};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::AppendHeaders;
use axum::{extract::Form, response::IntoResponse};
use serde::Deserialize;
use tracing::trace;
use url::form_urlencoded::byte_serialize;

#[derive(Deserialize)]
pub struct TicketParams {
    ticket: String,
}

pub async fn save_session_get(
    State(s): State<AppState>,
    Query(tp): Query<TicketParams>,
) -> impl IntoResponse {
    trace!("ticket: {}", tp.ticket);
    let token = gen_access_token(s.session_secret.as_bytes(), tp.ticket);
    (
        StatusCode::FOUND,
        AppendHeaders([
            (LOCATION, "/".to_string()),
            (SET_COOKIE, format!("x-token={};path=/", token)),
        ]),
        "success",
    )
}

pub async fn login_get(Query(q): Query<MsgParams>) -> impl IntoResponse {
    let template = PageTemplate {
        msg: q.msg.unwrap_or("".to_string()),
    };
    common::HtmlTemplate(template)
}

#[derive(Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct MsgParams {
    msg: Option<String>,
}

pub async fn login_post(
    State(mut s): State<AppState>,
    Form(v): Form<LoginParams>,
) -> impl IntoResponse {
    let ok = s
        .ldap
        .bind(&v.username, &v.password)
        .await
        .map_err(|e| {
            let msg: String = byte_serialize("登陆失败".as_bytes()).collect();
            (
                StatusCode::OK,
                ([("HX-Redirect", format!("{}{}", "?msg=", msg))]),
            )
        })
        .unwrap();
    if !ok {
        let msg: String = byte_serialize("登陆失败".as_bytes()).collect();
        return (
            StatusCode::OK,
            ([("HX-Redirect", format!("{}{}", "?msg=", msg))]),
        );
    }
    let token = gen_exchange_token(s.session_secret.as_bytes(), &v.username);
    let q: String = byte_serialize(token.as_bytes()).collect();
    (
        StatusCode::OK,
        ([("HX-Redirect", format!("{}{}", "/auth-code?ticket=", q))]),
    )
}

#[derive(Template)]
#[template(path = "login.html")]
struct PageTemplate {
    msg: String,
}
