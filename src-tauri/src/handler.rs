use crate::{
    project::{Project, ProjectReq},
    task::Task,
};

#[tauri::command(async)]
pub async fn projects(keyword: Option<String>) -> Vec<Project> {
    let mut req = ProjectReq::default();
    req.name = keyword;

    Project::fetch_all(&req).await.unwrap_or(vec![])
}

#[tauri::command(async)]
pub async fn save_project(name: String, cover: Option<String>) -> Project {
    let mut project = Project::new(&name);
    project.insert().await.unwrap()
}

#[tauri::command(async)]
pub async fn del_project(id: u64) -> Project {
    unimplemented!()
}

#[tauri::command(async)]
pub async fn save_task() -> Task {
    unimplemented!()
}

#[tauri::command(async)]
pub async fn del_task() -> Task {
    unimplemented!()
}
