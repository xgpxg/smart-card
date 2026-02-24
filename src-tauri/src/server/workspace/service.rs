use crate::db::models::workspace::{
    Font, Pagination, TransTextStatus, Workspace, WorkspaceBuilder,
};
use crate::db::{tools, Pool};
use common::id;
use rbs::value;
use std::fs::File;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};

pub(crate) async fn list() -> anyhow::Result<Vec<Workspace>> {
    let mut list = Workspace::select_all(Pool::get()?).await?;
    list.sort_by(|a, b| b.id.cmp(&a.id));
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
        .trans_text_status(Some(TransTextStatus::NotStart))
        .font(Some(Font::default()))
        .pagination(Some(Pagination::default()))
        .create_time(Some(tools::now()))
        .build()?;

    Workspace::insert(Pool::get()?, &ws).await?;
    Ok(())
}

pub(crate) async fn update(mut workspace: Workspace) -> anyhow::Result<()> {
    workspace.pagination = Some(Pagination {
        delimiter: workspace
            .pagination
            .clone()
            .unwrap()
            .delimiter
            .replace("\n", "\\\\n"),
        ..workspace.pagination.unwrap()
    });
    log::info!("更新数据: {:?}", workspace);
    Workspace::update_by_map(
        Pool::get()?,
        &workspace,
        value! {
            "id": workspace.id,
        },
    )
    .await?;
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
        return Err(anyhow::anyhow!("Not Found"));
    }
    let ws = ws.first().unwrap().clone();
    Ok(ws)
}

pub(crate) async fn start_audio_to_text(id: i64) -> anyhow::Result<()> {
    let mut workspace = detail(id).await?;
    let path = &workspace.file_path.clone().unwrap();
    log::info!("开始识别文件: {}", path);

    let tx = Pool::get()?;

    let mut update = Workspace::default();
    update.trans_text_status = Some(TransTextStatus::Processing);
    Workspace::update_by_map(
        tx,
        &workspace,
        value! {
            "id": id,
        },
    )
    .await?;
    let content = asr::run(path).await?;

    let mut update = Workspace::default();
    update.id = Some(id);
    update.trans_text = Some(content);
    update.trans_text_status = Some(TransTextStatus::Ok);
    log::info!("识别完成: {}", path);
    Workspace::update_by_map(
        tx,
        &update,
        value! {
            "id": id,
        },
    )
    .await?;

    Ok(())
}
