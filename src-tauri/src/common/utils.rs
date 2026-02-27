use crate::common::res::Res;
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
