use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskPriority {
    Urgency,
    High,
    Mid,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Delayed,
    Ended,
    Expired,
}

/// 任务
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u64,
    project_id:u64,
    /// 任务标题
    title: String,
    /// 任务描述
    desc: Option<String>,
    /// 优先级
    priority: TaskPriority,
    tags: Vec<String>,
    /// 开始时间
    start_time: u64,
    /// 结束时间
    end_time: u64,
    /// 状态
    status: TaskStatus,
    /// 子任务
    tasks: Vec<SubTask>,

    create_at: Option<u64>,
    update_at: Option<u64>,
    delete_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubTask {
    id: u64,
    /// 任务标题
    title: String,
    /// 开始时间
    start_time: u64,
    /// 结束时间
    end_time: u64,
    /// 状态
    status: TaskStatus,

    create_at: Option<u64>,
    update_at: Option<u64>,
    delete_at: Option<u64>,
}
