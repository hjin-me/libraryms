use axum::extract::{FromRef, FromRequestParts, TypedHeader};
use axum::headers::Cookie;
// use axum::headers::authorization::Bearer;
use crate::app::AppState;
use crate::data::accounts::{get_account_by_id, Role};
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::trace;

#[derive(Clone, Debug)]
pub struct Entity {
    pub uid: String,
    pub display_name: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,
    // Optional. Not Before (as UTC timestamp)
    nbf: usize,
    // Optional. Subject (whom token refers to)
    sub: String,
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct IdentRequire(pub Entity);

#[axum::async_trait]
impl<S> FromRequestParts<S> for IdentRequire
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match token_from_cookies(parts, state).await {
            Some(e) => Ok(Self(e)),
            None => Err((StatusCode::UNAUTHORIZED, "身份认证失败".to_string())),
        }
    }
}

pub struct IdentOptional(pub Option<Entity>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for IdentOptional
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(token_from_cookies(parts, state).await))
    }
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct IdentAdminRequire(pub Entity);

#[axum::async_trait]
impl<S> FromRequestParts<S> for IdentAdminRequire
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match token_from_cookies(parts, state).await {
            Some(e) => {
                if e.role == Role::Admin {
                    Ok(Self(e))
                } else {
                    Err((StatusCode::UNAUTHORIZED, "权限不够".to_string()))
                }
            }
            None => Err((StatusCode::UNAUTHORIZED, "身份认证失败".to_string())),
        }
        // You can either call them directly...
        // match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
        //     Ok(TypedHeader(Authorization(token_encoded))) => {
        //     }
        // }
    }
}

async fn token_from_cookies<S>(parts: &mut Parts, state: &S) -> Option<Entity>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    match TypedHeader::<Cookie>::from_request_parts(parts, state).await {
        Ok(TypedHeader(cookies)) => match cookies.get("x-token") {
            Some(token) => {
                let app_state = AppState::from_ref(state);
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(&app_state.session_secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(token) => {
                        match get_account_by_id(&app_state.pool, &token.claims.sub).await {
                            Ok(ac) => Some(Entity {
                                uid: ac.id,
                                display_name: ac.display_name,
                                role: ac.role,
                            }),
                            Err(_) => {
                                trace!("数据库查找用户失败");
                                None
                            }
                        }
                    }
                    Err(_) => None,
                }
            }
            None => None,
        },
        Err(_) => None,
    }
}

pub fn gen_exchange_token(secret: &[u8], sub: &String) -> String {
    // HS256
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            exp: time::OffsetDateTime::now_utc().unix_timestamp() as usize + 300,
            nbf: time::OffsetDateTime::now_utc().unix_timestamp() as usize - 300,
            sub: sub.clone(),
        },
        &EncodingKey::from_secret(secret),
    )
    .unwrap();

    token
}

pub fn gen_access_token(secret: &[u8], exchange_token: String) -> String {
    let token = decode::<Claims>(
        &exchange_token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .unwrap();

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            exp: time::OffsetDateTime::now_utc().unix_timestamp() as usize + 24 * 3600,
            nbf: time::OffsetDateTime::now_utc().unix_timestamp() as usize - 300,
            sub: token.claims.sub,
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
