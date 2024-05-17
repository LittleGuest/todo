use async_static::async_static;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

pub mod handler;
pub mod project;
pub mod snowflake;
pub mod task;

#[cfg(unix)]
// const DB_FILE: &str = "~/.local/share/p_todo/todo.sqlite";
const DB_FILE: &str = "../todo.sqlite";
#[cfg(windows)]
const DB_FILE: &str = "~/AppData/Local/p_todo/todo.sqlite";

pub fn now() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}

async_static! {
    static ref DB: Pool<Sqlite> = pool().await;
}

async fn pool() -> Pool<Sqlite> {
    sqlx::SqlitePool::connect("sqlite://../todo.sqlite")
        .await
        .unwrap()
}

/// 分页返回封装
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PageRes<T> {
    page: i64,
    page_size: i64,
    total: i64,
    list: Vec<T>,
    first: bool,
    last: bool,
    has_next: bool,
    has_pre: bool,
    total_pages: i64,
}

impl<T> std::default::Default for PageRes<T> {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 15,
            total: 0,
            list: vec![],
            first: true,
            last: false,
            has_next: false,
            has_pre: false,
            total_pages: 0,
        }
    }
}

impl<T> PageRes<T>
where
    T: Serialize + Clone,
{
    pub fn new(total: i64, mut page: i64, page_size: i64, list: &[T]) -> Self {
        if page <= 0 {
            page = 1;
        }
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Self {
            page,
            page_size,
            total,
            list: list.iter().cloned().collect::<Vec<_>>(),
            first: page == 1,
            last: page == total_pages,
            has_next: page < total_pages,
            has_pre: page > 1,
            total_pages,
        }
    }
}
