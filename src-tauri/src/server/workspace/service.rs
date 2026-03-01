use crate::db::models::workspace::{
    Font, Pagination, TransTextStatus, Workspace, WorkspaceBuilder,
};
use crate::db::{tools, Pool};
use crate::server::workspace::{
    check_is_audio_or_video, extract_audio_from_video, split_long_audio, FileType,
};
use common::id;
use rbatis::rbdc::uuid;
use rbs::value;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use tauri::{AppHandle, Emitter};

pub(crate) async fn list() -> anyhow::Result<Vec<Workspace>> {
    let mut list = Workspace::select_all(Pool::get()?).await?;
    list.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(list)
}

pub(crate) async fn add(file_path: String) -> anyhow::Result<()> {
    let file_type = check_is_audio_or_video(&file_path)?;
    let path = Path::new(&file_path);
    let filename = path.file_name().unwrap().to_string_lossy().to_string();
    let file_stem = path.file_stem().unwrap().to_string_lossy().to_string();
    let path = match file_type {
        FileType::Audio => Path::new(&file_path).to_path_buf(),
        FileType::Video => {
            let output_path = Path::new(&file_path)
                .parent()
                .unwrap()
                .join(format!("{}.mp3", file_stem));
            extract_audio_from_video(&file_path, &output_path.display().to_string())?;
            output_path
        }
    };
    let ws = WorkspaceBuilder::default()
        .id(Some(id::next()))
        .file_path(Some(path.display().to_string()))
        .file_name(Some(filename.clone()))
        .raw_file_name(Some(filename))
        .file_size(Some(path.metadata()?.len()))
        .file_type(Some(
            path.extension().unwrap().to_string_lossy().to_string(),
        ))
        .trans_text_status(Some(TransTextStatus::NotStart))
        .font(Some(Font::default()))
        .pagination(Some(Pagination::default()))
        .style_id(Some(1))
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
        &update,
        value! {
            "id": id,
        },
    )
    .await?;

    let file_size = workspace.file_size.unwrap();
    let content = if file_size <= 1024 * 1024 * 2 {
        asr::run(path).await?
    } else {
        let file_path = PathBuf::from(path);
        let tem_dir = env::temp_dir().join(uuid::Uuid::new().to_string());
        fs::create_dir_all(&tem_dir)?;
        log::info!(
            "开始拆分音频: {} -> {}",
            file_path.display(),
            tem_dir.display()
        );
        let files = split_long_audio(
            &file_path.display().to_string(),
            &tem_dir.display().to_string(),
            60,
        )?;

        log::info!("音频拆分完成");

        let mut content = Vec::new();
        for file in files {
            log::info!("开始识别文件: {}", file);
            let content_part = asr::run(&file).await?;
            content.push(content_part);
        }

        fs::remove_dir_all(tem_dir)?;

        content.join("")
    };

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
