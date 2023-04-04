use anyhow::Result;
#[cfg(feature = "ssr")]
use once_cell::sync::OnceCell;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
static INSTANCE: OnceCell<Arc<PgPool>> = OnceCell::new();

#[cfg(feature = "ssr")]
pub async fn init(pg_dsn: &str) -> Result<()> {
    let pool = PgPool::connect(pg_dsn).await?;
    INSTANCE.set(Arc::new(pool)).expect("初始化数据库连接失败");
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_client() -> Result<Arc<PgPool>> {
    match INSTANCE.get() {
        Some(c) => Ok(c.clone()),
        None => Err(anyhow::anyhow!("数据库连接未初始化")),
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[tokio::test]
//     async fn es() {
//         init("http://127.0.0.1:9200").await.unwrap();
//         let client = get_client().await.unwrap();
//         let rand_index = format!("test_{}", time::OffsetDateTime::now_utc().unix_timestamp());
//     }
// }
