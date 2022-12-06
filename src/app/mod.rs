mod auth;
mod books;
mod common;
mod home;
pub mod ident;

use crate::app::books::{book_list_get, borrow_book, delete_book, return_book, simple_storage};
use crate::app::ident::{login_get, login_post, save_session_get};
use crate::data::books::BookMS;
use crate::data::ldap::LdapIdent;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::Router;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
    pub session_secret: String,
    pub ldap: LdapIdent,
    pub book_ms: BookMS,
}

pub async fn start(
    pg_pool: &Pool<PostgresConnectionManager<NoTls>>,
    session_secret: &String,
    ldap: &mut LdapIdent,
    book_ms: &BookMS,
) {
    let app_state = AppState {
        pool: pg_pool.clone(),
        session_secret: session_secret.clone(),
        ldap: ldap.clone(),
        book_ms: book_ms.clone(),
    };
    // // build our application with a single route
    let app = Router::new()
        .route("/", get(home::home))
        .route("/liveness", get(|| async { "I'm alive!" }))
        .route("/readiness", get(|| async { "I'm ready!" }))
        .route("/authentication", get(login_get).post(login_post))
        .route("/auth-code", get(save_session_get))
        .route("/books", get(book_list_get))
        .route("/book/fast-import", post(simple_storage))
        .route("/book/:book_id", delete(delete_book))
        .route("/book/borrow/:book_id", post(borrow_book))
        .route("/book/return/:book_id", post(return_book))
        .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(Pool<PostgresConnectionManager<NoTls>>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        Ok(Self(app_state.pool))
    }
}
