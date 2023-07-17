use crate::{project::Project, task::Task};

#[tauri::command]
pub fn projects(keyword: &str) -> Vec<Project> {
    dbg!(keyword);
    vec![Project::new("测试项目1", None); 5]
}

#[tauri::command]
pub fn save_project(name: &str, cover: Option<String>) -> Project {
    Project::new(name, cover)
}

#[tauri::command]
pub fn del_project(id: u64) -> Project {
    unimplemented!()
}

#[tauri::command]
pub fn save_task() -> Task {
    unimplemented!()
}

#[tauri::command]
pub fn del_task() -> Task {
    unimplemented!()
}
