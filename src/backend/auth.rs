use crate::backend::conf::Config;
use cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use leptos_reactive::use_context;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::string::ToString;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,
    // Optional. Not Before (as UTC timestamp)
    nbf: usize,
    // Optional. Subject (whom token refers to)
    sub: String,
}

const COOKIE_NAME: &'static str = "x-token";

pub async fn account_info_from_cookies(cx: leptos::Scope) -> Option<AccountInfo> {
    let rp = match use_context::<leptos_axum::RequestParts>(cx) {
        Some(rp) => rp,
        None => return None,
    };
    debug!("request parts: {:?}", &rp);
    let h = match rp.headers.get(http::header::COOKIE) {
        Some(r) => String::from_utf8_lossy(r.as_bytes()).to_string(),
        None => return None,
    };
    debug!("cookies: {:?}", &h);
    let token = match Cookie::split_parse(h).find(|cookie| match cookie {
        Ok(cookie) => cookie.name() == COOKIE_NAME,
        Err(_) => false,
    }) {
        Some(c) => match c {
            Ok(c) => c,
            Err(_) => return None,
        },
        None => return None,
    }
    .value()
    .to_string();
    debug!("token: {:?}", &token);

    let conf = match use_context::<Config>(cx) {
        Some(rp) => rp,
        None => return None,
    };
    debug!("conf: {:?}", &conf);
    let pool = match crate::backend::db::from_scope(cx) {
        Ok(pool) => pool,
        Err(_) => return None,
    };
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(conf.session_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token) => match get_account_by_id(&pool, &token.claims.sub).await {
            Ok(ac) => Some(ac),
            Err(_) => {
                debug!("数据库查找用户失败");
                None
            }
        },
        Err(e) => {
            debug!("token解析失败: {:?}", e);
            None
        }
    }
}
pub async fn try_add_new_account(
    cx: leptos::Scope,
    id: &str,
    display_name: &str,
) -> anyhow::Result<()> {
    let pool = crate::backend::db::from_scope(cx)?;
    sqlx::query(
        r#"
        INSERT INTO accounts (id, display_name, role, created_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(id)
    .bind(display_name)
    .bind(Role::User.to_string())
    .bind(time::OffsetDateTime::now_utc())
    .execute(pool.as_ref())
    .await?;
    Ok(())
}
pub fn set_account_info(cx: leptos::Scope, sub: &str) -> anyhow::Result<()> {
    let conf = use_context::<Config>(cx).ok_or(anyhow::anyhow!("配置文件不存在"))?;
    let token = gen_access_token(conf.session_secret.as_bytes(), sub);
    let mut c = Cookie::new(COOKIE_NAME, token);
    c.set_max_age(time::Duration::days(7));
    c.set_path("/");
    if let Some(r) = use_context::<leptos_axum::ResponseOptions>(cx) {
        r.insert_header(http::header::SET_COOKIE, c.to_string().parse()?)
    };
    Ok(())
}

pub fn gen_access_token(secret: &[u8], sub: &str) -> String {
    // HS256
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            exp: time::OffsetDateTime::now_utc().unix_timestamp() as usize + 24 * 3600,
            nbf: time::OffsetDateTime::now_utc().unix_timestamp() as usize - 300,
            sub: sub.to_string(),
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap();

    token
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn token() {
        // HS256
        let token = encode(
            &Header::default(),
            &Claims {
                exp: time::OffsetDateTime::now_utc().unix_timestamp() as usize + 3600,
                nbf: 1,
                sub: "this is sub".to_string(),
            },
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap();
        // assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjAsIm5iZiI6MCwic3ViIjoidGhpcyBpcyBzdWIifQ.UU9d5Uxp28yG-OzklVAz42y28IKjpSy9ElwZRy-cwZk".to_string(), token);

        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .unwrap();
        assert_eq!("this is sub".to_string(), token.claims.sub)
    }
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

#[derive(Debug)]
pub struct AccountInfo {
    pub id: String,
    pub display_name: String,
    pub role: Role,
}
pub async fn get_account_by_id(pool: &PgPool, id: &str) -> anyhow::Result<AccountInfo> {
    let rs = sqlx::query("SELECT id, display_name, role FROM accounts WHERE id = $1 LIMIT 1")
        .bind(&id)
        .fetch_one(pool)
        .await?;
    Ok(AccountInfo {
        id: rs.get(0),
        display_name: rs.get(1),
        role: Role::from_str(rs.get(2)).unwrap_or(Role::User),
    })
}
