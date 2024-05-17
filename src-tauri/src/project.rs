use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::DB;

///
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    FromRow,
    Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Project {
    /// id
    pub id: Option<i64>,
    /// name
    #[validate(length(max = 255))]
    pub name: Option<String>,
    /// cover
    #[validate(length(max = 255))]
    pub cover: Option<String>,
    /// start_time
    pub start_time: Option<i64>,
    /// end_time
    pub end_time: Option<i64>,
    /// status
    pub status: Option<i64>,
    /// create_at
    pub create_at: Option<i64>,
    /// update_at
    pub update_at: Option<i64>,
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..Default::default()
        }
    }
}

impl Project {
    fn table_name() -> String {
        "project".to_string()
    }

    fn columns() -> String {
        "id,name,cover,start_time,end_time,status,create_at,update_at".to_string()
    }

    pub async fn fetch_by_id(id: i64) -> anyhow::Result<Self> {
        let sql = format!(
            "select {} from {} where id = ?",
            Self::columns(),
            Self::table_name()
        );
        Ok(sqlx::query_as::<_, Self>(&sql)
            .bind(id)
            .fetch_one(DB.await)
            .await?)
    }

    pub async fn fetch_all(req: &ProjectReq) -> anyhow::Result<Vec<Self>> {
        let mut sql = format!("select {} from {}", Self::columns(), Self::table_name());

        let mut where_sql = " WHERE 1=1 ".to_string();

        if let Some(id) = &req.id {
            where_sql.push_str(&format!(" and {} = {} ", "id", id));
        }

        if let Some(name) = &req.name {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "name", name));
        }

        if let Some(cover) = &req.cover {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "cover", cover));
        }

        if let Some(start_time) = &req.start_time {
            where_sql.push_str(&format!(" and {} = {} ", "start_time", start_time));
        }

        if let Some(end_time) = &req.end_time {
            where_sql.push_str(&format!(" and {} = {} ", "end_time", end_time));
        }

        if let Some(status) = &req.status {
            where_sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(create_at) = &req.create_at {
            where_sql.push_str(&format!(" and {} = {} ", "create_at", create_at));
        }

        if let Some(update_at) = &req.update_at {
            where_sql.push_str(&format!(" and {} = {} ", "update_at", update_at));
        }

        sql.push_str(&where_sql);

        Ok(sqlx::query_as::<_, Self>(&sql).fetch_all(DB.await).await?)
    }

    pub async fn insert(&mut self) -> anyhow::Result<Self> {
        let sql = format!(
            "INSERT INTO {} ({}) VALUES({})",
            Self::table_name(),
            Self::columns(),
            "?,?,?,?,?,?,?,?,".trim_end_matches(',')
        );
        let id = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.cover)
            .bind(&self.start_time)
            .bind(&self.end_time)
            .bind(&self.status)
            .bind(&self.create_at)
            .bind(&self.update_at)
            .execute(DB.await)
            .await?
            .last_insert_rowid();
        Self::fetch_by_id(id).await
    }

    pub async fn update(&mut self) -> anyhow::Result<bool> {
        let sql = format!(
            "UPDATE {} set account = ?, set {} where id = ?",
            Self::table_name(),
            "id = ?,name = ?,cover = ?,start_time = ?,end_time = ?,status = ?,create_at = ?,update_at = ?,".trim_end_matches(',')
        );
        let res = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.cover)
            .bind(&self.start_time)
            .bind(&self.end_time)
            .bind(&self.status)
            .bind(&self.create_at)
            .bind(&self.update_at)
            .bind(&self.id)
            .execute(DB.await)
            .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn delete(&self) -> anyhow::Result<bool> {
        let sql = format!("DELETE FROM {} WHERE id = ?", Self::table_name());
        let res = sqlx::query(&sql).bind(self.id).execute(DB.await).await?;
        Ok(res.rows_affected() > 0)
    }

    async fn count(where_sql: &str) -> anyhow::Result<(i64,)> {
        let count_sql = format!(
            "SELECT count(*) FROM {} WHERE {}",
            Self::table_name(),
            where_sql
        );
        Ok(sqlx::query_as::<_, (i64,)>(&count_sql)
            .fetch_one(DB.await)
            .await?)
    }

    pub async fn page(req: &ProjectReq) -> anyhow::Result<super::PageRes<Self>> {
        let mut where_sql = " 1 = 1 ".to_string();

        if let Some(id) = &req.id {
            where_sql.push_str(&format!(" and {} = {} ", "id", id));
        }

        if let Some(name) = &req.name {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "name", name));
        }

        if let Some(cover) = &req.cover {
            where_sql.push_str(&format!(" and {} like '%{}%' ", "cover", cover));
        }

        if let Some(start_time) = &req.start_time {
            where_sql.push_str(&format!(" and {} = {} ", "start_time", start_time));
        }

        if let Some(end_time) = &req.end_time {
            where_sql.push_str(&format!(" and {} = {} ", "end_time", end_time));
        }

        if let Some(status) = &req.status {
            where_sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(create_at) = &req.create_at {
            where_sql.push_str(&format!(" and {} = {} ", "create_at", create_at));
        }

        if let Some(update_at) = &req.update_at {
            where_sql.push_str(&format!(" and {} = {} ", "update_at", update_at));
        }

        let (count,) = Self::count(&where_sql).await?;

        let page_size = req.page_size.unwrap_or(20);
        let mut page = req.page.unwrap_or(0) - 1;
        if page < 0 {
            page = 0;
        }
        where_sql.push_str(&format!(" LIMIT {}, {} ", page * page_size, page_size));

        let res = match count > 0 {
            true => {
                let mut sql = format!(
                    "SELECT {} FROM {} WHERE ",
                    Self::columns(),
                    Self::table_name()
                );

                sql.push_str(&where_sql);
                sqlx::query_as::<_, Self>(&sql).fetch_all(DB.await).await?
            }
            false => Vec::new(),
        };
        Ok(super::PageRes::new(count, page, page_size, &res))
    }
}

///
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    FromRow,
    Validate,
)]
pub struct ProjectReq {
    pub time_type: Option<u8>,
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,

    /// id
    pub id: Option<i64>,
    /// name
    pub name: Option<String>,
    /// cover
    pub cover: Option<String>,
    /// start_time
    pub start_time: Option<i64>,
    /// end_time
    pub end_time: Option<i64>,
    /// status
    pub status: Option<i64>,
    /// create_at
    pub create_at: Option<i64>,
    /// update_at
    pub update_at: Option<i64>,
}
