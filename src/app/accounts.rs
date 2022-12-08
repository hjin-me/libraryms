use crate::app::auth::IdentRequire;
use crate::app::AppState;
use crate::data::accounts::{Account, Role};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn sync_accounts(
    IdentRequire(_): IdentRequire,
    State(mut s): State<AppState>,
) -> impl IntoResponse {
    let acs = s.ldap.all_accounts().await.expect("获取全部LDAP账户失败");
    let ac = Account::new(&s.pool);
    for a in acs {
        ac.add(&a.uid, &a.display_name, &Role::User)
            .await
            .expect("添加账户失败");
    }
    (StatusCode::OK, "同步完成")
}
