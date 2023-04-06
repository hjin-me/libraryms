use crate::entity::{Book, BookState};
use leptos::ServerFnError::ServerError;
use leptos::*;
use leptos_router::ToHref;
use serde::{Deserialize, Serialize};
use tracing::trace;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    let _ = FastStorageBook::register();
    let _ = BookList::register();
    let _ = BookDetail::register();
}
#[server(FastStorageBook, "/api")]
pub async fn fast_storage_book(isbn: String) -> Result<(), ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    bms.storage(isbn.as_str(), "songsong")
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    Ok(())
}

#[server(BookList, "/api")]
pub async fn book_list(offset: i64, limit: i64) -> Result<Vec<BookUI>, ServerFnError> {
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let books = bms
        .list(&limit, &offset)
        .await
        .map_err(|e| ServerError(e.to_string()))?
        .iter()
        .map(|b| BookUI::from(b))
        .collect();

    Ok(books)
}

#[server(BookDetail, "/api")]
pub async fn book_detail(id: i64) -> Result<BookUI, ServerFnError> {
    trace!("id: {}", id);
    let bms = crate::backend::books::get_bms()
        .await
        .map_err(|e| ServerError(e.to_string()))?;
    let mut book: BookUI = bms
        .get_one_by_id(&id)
        .await
        .map_err(|e| ServerError(e.to_string()))?
        .into();

    book.bind_role(&Some(UserSession {
        uid: "songsong".to_string(),
        display_name: "SS".to_string(),
        role: Role::Admin,
    }));

    Ok(book)
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
impl From<Book> for BookUI {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            isbn: value.isbn,
            title: value.title,
            authors: value.authors,
            publisher: value.publisher,
            import_at: value.import_at,
            state: value.state,
            operator: value.operator,
            operator_name: value.operator_name,
            operate_at: value.operate_at,
            thumbnail: value.thumbnail,
            actions: vec![],
        }
    }
}
#[cfg(feature = "ssr")]
impl From<&Book> for BookUI {
    fn from(value: &Book) -> Self {
        Self::from(value.clone())
    }
}
#[derive(Clone, Debug)]
pub struct UserSession {
    pub uid: String,
    pub display_name: String,
    pub role: Role,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Role {
    Admin,
    User,
}

impl BookUI {
    pub fn bind_role(&mut self, current_user: &Option<UserSession>) {
        // let act_borrow = BookAction {
        //     act: "borrow".to_string(),
        //     // btn_type: "btn-primary".to_string(),
        //     // text: "借阅".to_string(),
        // };
        // let act_return = BookAction {
        //     act: "return".to_string(),
        //     // btn_type: "btn-success".to_string(),
        //     // text: "归还".to_string(),
        // };
        // let act_confirm = BookAction {
        //     act: "confirm".to_string(),
        //     // btn_type: "btn-info".to_string(),
        //     // text: "确认已归还".to_string(),
        // };
        // let act_lost = BookAction {
        //     act: "lost".to_string(),
        //     // btn_type: "btn-dark".to_string(),
        //     // text: "标记遗失".to_string(),
        // };
        // let act_reset = BookAction {
        //     act: "reset".to_string(),
        //     // btn_type: "btn-secondary".to_string(),
        //     // text: "重置状态".to_string(),
        // };
        // let act_delete = BookAction {
        //     act: "delete".to_string(),
        //     // btn_type: "btn-danger".to_string(),
        //     // text: "删除".to_string(),
        // };
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
