use crate::common::res::Res;
use crate::db::models::workspace::Workspace;
use crate::server::workspace::service;

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn list_workspaces() -> Res<Vec<Workspace>> {
    match service::list().await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn add_workspace(file_path: String) -> Res<()> {
    match service::add(file_path).await {
        Ok(()) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn delete_workspace(id: i64) -> Res<()> {
    match service::delete(id).await {
        Ok(()) => Res::success(()),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub(crate) async fn get_workspace(id: i64) -> Res<Workspace> {
    match service::detail(id).await {
        Ok(res) => Res::success(res),
        Err(e) => Res::error(e.to_string().as_str()),
    }
}
