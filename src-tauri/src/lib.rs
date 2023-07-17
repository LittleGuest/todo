use async_static::async_static;
use sqlx::{Pool, Sqlite};

pub mod handler;
pub mod project;
pub mod task;

async_static! {
    static ref DB: Pool<Sqlite> = pool().await;
}

async fn pool() -> Pool<Sqlite> {
    sqlx::sqlite::SqlitePool::connect("sqlite://::memory:")
        .await
        .unwrap()
}
