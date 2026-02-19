use crate::db;
use anyhow::Context;
use common::dir::AppDir;
use std::fs;
use tauri::App;

pub(crate) fn setup(app: &mut App) -> anyhow::Result<()> {
    tauri::async_runtime::block_on(async move {
        init().await?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

async fn init() -> anyhow::Result<()> {
    init_dir()?;
    // 初始化id生成器
    common::id::init();
    log::info!("ID生成器初始化完成");
    // 初始化数据库
    db::init().await?;
    log::info!("数据库初始化完成");

    Ok(())
}

fn init_dir() -> anyhow::Result<()> {
    let data_dir = AppDir::data_dir();
    fs::create_dir_all(data_dir).context("Failed to create data directory")?;

    fs::create_dir_all(data_dir.join("sqlite")).context("Failed to create sqlite directory")?;

    Ok(())
}
