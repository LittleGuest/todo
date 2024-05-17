use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use super::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Urgency,
    High,
    Mid,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Delayed,
    Ended,
    Expired,
}

/// 任务
#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow, Validate)]
pub struct Task {
    /// ID
    id: Option<u32>,
    /// 所属项目
    project_id: Option<u32>,
    /// PID
    pid: Option<u32>,
    /// 任务标题
    title: Option<String>,
    /// 任务描述
    desc: Option<String>,
    /// 优先级
    priority: Option<u32>,
    tags: Option<String>,
    /// 开始时间
    start_time: Option<u32>,
    /// 结束时间
    end_time: Option<u32>,
    /// 状态
    status: Option<u32>,
    /// 创建时间
    create_at: Option<u32>,
    /// 更新时间
    update_at: Option<u32>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

impl Task {
    fn table_name() -> String {
        "task".to_string()
    }

    fn columns() -> String {
        "id,project_id,pid,title,desc,priority,tags,start_time,end_time,status,create_at,update_at"
            .to_string()
    }

    pub async fn fetch_by_id(id: u32) -> anyhow::Result<Self> {
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

    pub async fn fetch_all() -> anyhow::Result<Vec<Self>> {
        let sql = format!("select {} from {}", Self::columns(), Self::table_name());

        Ok(sqlx::query_as::<_, Self>(&sql).fetch_all(DB.await).await?)
    }

    pub async fn insert(&mut self) -> anyhow::Result<Self> {
        let sql = format!(
            "INSERT INTO {} ({}) VALUES({})",
            Self::table_name(),
            Self::columns(),
            "?,?,?,?,?,?,?,?,?,?,".trim_end_matches(',')
        );
        let id = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.project_id)
            .bind(&self.project_id)
            .bind(&self.title)
            .bind(&self.desc)
            .bind(&self.priority)
            .bind(&self.tags)
            .bind(&self.start_time)
            .bind(&self.end_time)
            .bind(&self.status)
            .bind(&self.create_at)
            .bind(&self.update_at)
            .execute(DB.await)
            .await?
            .rows_affected();
        Self::fetch_by_id(id as u32).await
    }

    pub async fn update(&mut self) -> anyhow::Result<bool> {
        let sql = format!(
            "UPDATE {} set {} where id = ?",
            Self::table_name(),
            "id = ?,name = ?,cover = ?,start_time = ?,end_time = ?,status = ?,create_at = ?,update_at = ?,".trim_end_matches(',')
        );
        let res = sqlx::query(&sql)
            .bind(&self.id)
            .bind(&self.project_id)
            .bind(&self.project_id)
            .bind(&self.title)
            .bind(&self.desc)
            .bind(&self.priority)
            .bind(&self.tags)
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TaskTree {
    /// ID
    id: Option<u32>,
    /// 所属项目
    project_id: Option<u32>,
    /// PID
    pid: Option<u32>,
    /// 任务标题
    title: Option<String>,
    /// 任务描述
    desc: Option<String>,
    /// 优先级
    priority: Option<u32>,
    tags: Option<String>,
    /// 开始时间
    start_time: Option<u32>,
    /// 结束时间
    end_time: Option<u32>,
    /// 状态
    status: Option<u32>,
    /// 创建时间
    create_at: Option<u32>,
    /// 更新时间
    update_at: Option<u32>,

    children: Vec<TaskTree>,
}

impl From<&Task> for TaskTree {
    fn from(t: &Task) -> Self {
        Self {
            id: t.id,
            project_id: t.project_id,
            pid: t.pid,
            title: t.title.clone(),
            desc: t.desc.clone(),
            priority: t.priority,
            tags: t.tags.clone(),
            start_time: t.start_time,
            end_time: t.end_time,
            status: t.status,
            create_at: t.create_at,
            update_at: t.update_at,
            children: vec![],
        }
    }
}

impl TaskTree {
    pub async fn tree() -> anyhow::Result<Vec<TaskTree>> {
        let tasks = Task::fetch_all().await?;
        let tasks = tasks.iter().map(|t| t.into()).collect::<Vec<Self>>();
        let mut p = tasks
            .iter()
            .filter(|t| {
                if let Some(pid) = t.pid {
                    pid == 0
                } else {
                    true
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        p.iter_mut().for_each(|t| {
            t.children = tasks
                .iter()
                .filter(|c| c.pid == t.id)
                .cloned()
                .collect::<Vec<_>>();
        });
        Ok(p)
    }
}
