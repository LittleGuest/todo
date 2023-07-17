use serde::{Deserialize, Serialize};

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: u64,
    /// 项目名称
    name: String,
    /// 封面图
    cover: Option<String>,
    /// 开始时间
    start_time: u64,
    /// 结束时间
    end_time: u64,
    /// 状态
    status: Option<String>,

    create_at: Option<u64>,
    update_at: Option<u64>,
    delete_at: Option<u64>,
}

impl Project {
    pub fn new(name: &str, cover: Option<String>) -> Self {
        Self {
            id: 1,
            name: name.into(),
            cover,
            start_time: 11,
            end_time: 22,
            status: None,
            create_at: None,
            update_at: None,
            delete_at: None,
        }
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
