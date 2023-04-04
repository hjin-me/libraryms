use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use crate::errors::TodoAppError;
use axum::response::Response as AxumResponse;
use axum::{
    body::{boxed, Body, BoxBody},
    extract::Extension,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::{view, Errors, LeptosOptions};
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    uri: Uri,
    Extension(options): Extension<Arc<LeptosOptions>>,
    req: Request<Body>,
) -> AxumResponse {
    let options = &*options;
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let mut errors = Errors::default();
        errors.insert_with_default_key(TodoAppError::NotFound);
        let handler = leptos_axum::render_app_to_stream(
            options.to_owned(),
            move |cx| view! {cx, <ErrorTemplate outside_errors=errors.clone()/>},
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
