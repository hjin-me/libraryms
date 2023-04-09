use anyhow::Result;
use leptos_reactive::use_context;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::sync::Arc;

#[cfg(feature = "ssr")]
pub async fn init(pg_dsn: &str) -> Result<PgPool> {
    Ok(PgPool::connect(pg_dsn).await?)
}

#[cfg(feature = "ssr")]
pub fn from_scope(cx: leptos::Scope) -> Result<Arc<PgPool>> {
    Ok(use_context::<Arc<PgPool>>(cx).ok_or(anyhow::anyhow!("No pg context found"))?)
}
