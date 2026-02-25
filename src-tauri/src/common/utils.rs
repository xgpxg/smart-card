use crate::common::res::Res;
use anyhow::bail;
use std::io::Bytes;
use std::path::PathBuf;

#[tauri::command]
pub fn save_file(path: PathBuf, content: Vec<u8>) -> Res<()> {
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    }
    match std::fs::write(path, content) {
        Ok(_) => Res::success(()),
        Err(e) => Res::error(&e.to_string()),
    }
}

pub fn check_is_audio<P: Into<String>>(path: P) -> anyhow::Result<()> {
    let path = path.into();
    let ext = path.split(".").last().unwrap();
    if !matches!(ext, "mp3" | "wav" | "ogg" | "m4a") {
        return bail!("仅支持 mp3 | wav | ogg | m4a 格式的音频文件");
    }
    Ok(())
}
