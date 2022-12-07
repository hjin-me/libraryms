use crate::app::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

pub async fn liveness(State(s): State<AppState>) -> impl IntoResponse {
    info!("GET /liveness");
    s.pool.get().await.unwrap();
    "I'm alive!"
}
