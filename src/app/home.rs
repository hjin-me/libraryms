use crate::app::auth::{Entity, IdentOptional};
use askama::Template;
use axum::response::IntoResponse;
use tracing::info;

pub async fn home(IdentOptional(entity): IdentOptional) -> impl IntoResponse {
    info!("GET /");
    let template = PageTemplate {
        current_user: entity,
    };
    crate::app::common::HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "home.html")]
struct PageTemplate {
    current_user: Option<Entity>,
}
