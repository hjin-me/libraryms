use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BookState {
    Available,
    Borrowed,
    Returned,
    Lost,
    Deleted,
    Unknown,
}
impl BookState {
    pub fn to_string(&self) -> String {
        match self {
            BookState::Available => "可借阅".to_string(),
            BookState::Borrowed => "已借出".to_string(),
            BookState::Returned => "已归还".to_string(),
            BookState::Lost => "遗失".to_string(),
            BookState::Deleted => "已删除".to_string(),
            BookState::Unknown => "未知".to_string(),
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "可借阅" => BookState::Available,
            "已借出" => BookState::Borrowed,
            "已归还" => BookState::Returned,
            "遗失" => BookState::Lost,
            "已删除" => BookState::Deleted,
            _ => BookState::Unknown,
        }
    }
}
#[cfg(feature = "ssr")]
impl From<crate::backend::books::BookStateModel> for BookState {
    fn from(value: crate::backend::books::BookStateModel) -> Self {
        match value {
            crate::backend::books::BookStateModel::Available => BookState::Available,
            crate::backend::books::BookStateModel::Borrowed => BookState::Borrowed,
            crate::backend::books::BookStateModel::Returned => BookState::Returned,
            crate::backend::books::BookStateModel::Lost => BookState::Lost,
            crate::backend::books::BookStateModel::Deleted => BookState::Deleted,
            crate::backend::books::BookStateModel::Unknown => BookState::Unknown,
        }
    }
}
#[cfg(feature = "ssr")]
impl From<&crate::backend::books::BookStateModel> for BookState {
    fn from(value: &crate::backend::books::BookStateModel) -> Self {
        value.clone().into()
    }
}
impl fmt::Display for BookState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
