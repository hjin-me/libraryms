use crate::api::auth::{get_account, Role, UserSession};
use crate::api::entity::BookState;
use leptos::ServerFnError::{Request, ServerError};
use leptos::*;
use serde::{Deserialize, Serialize};
// use tracing::trace;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = FastStorageBook::register();
    let _ = BookList::register();
    let _ = BookDetail::register();
    let _ = BorrowBook::register();
    let _ = ReturnBook::register();
    let _ = ConfirmReturnBook::register();
}
#[server(FastStorageBook, "/api")]
pub async fn fast_storage_book(cx: Scope, isbn: String) -> Result<(), ServerFnError> {
    let ac = get_account(cx)
        .await?
        .ok_or(Request("Not logged in".to_string()))?;
    if ac.role != Role::Admin {
        return Err(Request("Not admin".to_string()));
    }
    let bms = crate::backend::books::BookMS::from_scope(cx);
    bms.storage(isbn.as_str(), &ac.uid)
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}

#[server(BookList, "/api")]
pub async fn book_list(
    cx: Scope,
    offset: Option<i64>,
    limit: Option<i64>,
    q: Option<String>,
) -> Result<Vec<BookUI>, ServerFnError> {
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);
    let ac = get_account(cx).await?;
    // dbg!(q);
    let bms = crate::backend::books::BookMS::from_scope(cx);
    let books = bms
        .list(&limit, &offset, &q)
        .await
        .map_err(|e| ServerError(e.to_string()))?
        .iter()
        .map(|b| {
            let mut b = BookUI::from(b);
            b.bind_role(&ac);
            b
        })
        .collect();

    Ok(books)
}

#[server(BookDetail, "/api")]
pub async fn book_detail(cx: Scope, id: i64) -> Result<BookUI, ServerFnError> {
    let bms = crate::backend::books::BookMS::from_scope(cx);
    let mut book: BookUI = bms
        .get_one_by_id(&id)
        .await
        .map_err(|e| ServerError(e.to_string()))?
        .into();
    let ac = get_account(cx).await?;

    book.bind_role(&ac);

    Ok(book)
}

#[server(BorrowBook, "/api")]
pub async fn borrow_book(cx: Scope, id: i64) -> Result<(), ServerFnError> {
    let ac = get_account(cx)
        .await?
        .ok_or(Request("Not login".to_string()))?;
    let bms = crate::backend::books::BookMS::from_scope(cx);
    bms.borrow(&id, ac.uid.as_str())
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}
#[server(ReturnBook, "/api")]
pub async fn return_book(cx: Scope, id: i64) -> Result<(), ServerFnError> {
    let ac = get_account(cx)
        .await?
        .ok_or(Request("Not login".to_string()))?;
    let bms = crate::backend::books::BookMS::from_scope(cx);
    bms.revert_to(&id, ac.uid.as_str())
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}
#[server(ConfirmReturnBook, "/api")]
pub async fn confirm_return_book(cx: Scope, id: i64) -> Result<(), ServerFnError> {
    let ac = get_account(cx)
        .await?
        .ok_or(Request("Not login".to_string()))?;
    if ac.role != Role::Admin {
        return Err(Request("Not admin".to_string()));
    }
    let bms = crate::backend::books::BookMS::from_scope(cx);
    bms.confirm(&id, &ac.uid)
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookUI {
    pub id: i64,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub import_at: time::OffsetDateTime,
    pub state: BookState,
    pub operator: String,
    pub operator_name: String,
    pub operate_at: time::OffsetDateTime,
    pub thumbnail: String,
    pub actions: Vec<BookAction>,
}

#[cfg(feature = "ssr")]
impl From<crate::backend::books::BookModel> for BookUI {
    fn from(value: crate::backend::books::BookModel) -> Self {
        Self {
            id: value.id,
            isbn: value.isbn.unwrap_or("".to_string()),
            title: value.title,
            authors: value.authors,
            publisher: value.publisher.unwrap_or("".to_string()),
            import_at: value.created_at,
            state: value.state.into(),
            operator: value.operator,
            operator_name: value.operator_name,
            operate_at: value.operate_at,
            thumbnail: value.thumbnail.unwrap_or("".to_string()),
            actions: vec![],
        }
    }
}
#[cfg(feature = "ssr")]
impl From<&crate::backend::books::BookModel> for BookUI {
    fn from(value: &crate::backend::books::BookModel) -> Self {
        Self::from(value.clone())
    }
}

impl BookUI {
    pub fn bind_role(&mut self, current_user: &Option<UserSession>) {
        let current_uid = current_user
            .as_ref()
            .map(|u| u.uid.clone())
            .unwrap_or("".to_string());
        let role = current_user
            .as_ref()
            .map(|u| u.role.clone())
            .unwrap_or(Role::User);
        let mut actions = vec![];
        if self.state == BookState::Available && current_uid != "" {
            actions.push(BookAction::Borrow);
        }
        if self.state == BookState::Borrowed && current_uid == self.operator {
            actions.push(BookAction::Return);
        }
        if self.state == BookState::Returned && role == Role::Admin {
            actions.push(BookAction::Confirm);
        }
        if self.state == BookState::Lost && role == Role::Admin {
            actions.push(BookAction::Reset);
        }
        if role == Role::Admin {
            actions.push(BookAction::Lost);
            actions.push(BookAction::Delete);
        }
        self.actions = actions;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BookAction {
    Borrow,
    Return,
    Confirm,
    Lost,
    Reset,
    Delete,
}
