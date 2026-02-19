use crate::db::models::workspace::{Workspace, WorkspaceBuilder};
use crate::db::Pool;
use common::id;
use rbs::value;
use std::fs::File;
use std::path::{Path, PathBuf};

pub(crate) async fn list() -> anyhow::Result<Vec<Workspace>> {
    let list = Workspace::select_all(Pool::get()?).await?;
    Ok(list)
}

pub(crate) async fn add(file_path: String) -> anyhow::Result<()> {
    let path = Path::new(&file_path);
    let ws = WorkspaceBuilder::default()
        .id(Some(id::next()))
        .file_path(Some(path.display().to_string()))
        .file_name(Some(
            path.file_name().unwrap().to_string_lossy().to_string(),
        ))
        .raw_file_name(Some(
            path.file_name().unwrap().to_string_lossy().to_string(),
        ))
        .file_size(Some(path.metadata()?.len()))
        .file_type(Some(
            path.extension().unwrap().to_string_lossy().to_string(),
        ))
        .build()?;

    Workspace::insert(Pool::get()?, &ws).await?;
    Ok(())
}

pub(crate) async fn delete(id: i64) -> anyhow::Result<()> {
    Workspace::delete_by_map(
        Pool::get()?,
        value! {
            "id": id
        },
    )
    .await?;
    Ok(())
}

// 查询详情
pub(crate) async fn detail(id: i64) -> anyhow::Result<Workspace> {
    let ws = Workspace::select_by_map(
        Pool::get()?,
        value! {
            "id": id
        },
    )
    .await?;
    if ws.is_empty() {
        return Err(anyhow::anyhow!("未找到该文件"));
    }
    let ws = ws.first().unwrap().clone();
    Ok(ws)
}
