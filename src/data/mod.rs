pub mod books;
pub mod ldap;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub async fn get_pool(pg_dsn: &String) -> Result<Pool<PostgresConnectionManager<NoTls>>, String> {
    // set up connection pool
    let manager = PostgresConnectionManager::new_from_stringlike(pg_dsn, NoTls).unwrap();
    Pool::builder()
        .build(manager)
        .await
        .map_err(|e| e.to_string())
}
