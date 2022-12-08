use crate::data;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

struct AccountModel {
    pub id: String,
    pub display_name: String,
    pub role: String,
    pub created_at: time::OffsetDateTime,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Admin,
    User,
}
impl Role {
    pub fn from_str(s: &str) -> Result<Role, String> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _ => Err(format!("invalid role: {}", s)),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Role::Admin => "admin".to_string(),
            Role::User => "user".to_string(),
        }
    }
}

pub struct Account {
    pg: Pool<PostgresConnectionManager<NoTls>>,
}

impl Account {
    pub fn new(pg: &Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { pg: pg.clone() }
    }

    pub async fn add(
        &self,
        uid: &str,
        display_name: &str,
        role: &Role,
    ) -> Result<(), data::error::Error> {
        let conn = self
            .pg
            .get()
            .await
            .map_err(|e| data::error::with_msg(Some(e), "获取数据库连接失败"))?;
        conn.execute(
            "INSERT INTO accounts (id, display_name, role, created_at) VALUES ($1, $2, $3, $4)",
            &[
                &uid,
                &display_name,
                &role.to_string(),
                &time::OffsetDateTime::now_utc(),
            ],
        )
        .await
        .map_err(|e| data::error::with_msg(Some(e), "插入账户失败"))?;
        Ok(())
    }
}
pub struct AccountInfo {
    pub id: String,
    pub display_name: String,
    pub role: Role,
}
pub async fn get_account_by_id(
    pool: &Pool<PostgresConnectionManager<NoTls>>,
    id: &str,
) -> Result<AccountInfo, data::error::Error> {
    let conn = pool
        .get()
        .await
        .map_err(|e| data::error::with_msg(Some(e), "获取数据库连接失败"))?;
    let rs = conn
        .query_one(
            "SELECT id, display_name, role FROM accounts WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|e| data::error::with_msg(Some(e), "查询账户失败"))?;
    Ok(AccountInfo {
        id: rs.get(0),
        display_name: rs.get(1),
        role: Role::from_str(rs.get::<_, &str>(2)).unwrap_or(Role::User),
    })
}
