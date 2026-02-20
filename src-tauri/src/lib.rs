mod common;
mod config;
mod db;
mod server;
mod setup;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    config::log_config::init_log();

    tauri::Builder::default()
        .setup(|app| {
            setup::setup(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            server::workspace::commands::list_workspaces,
            server::workspace::commands::add_workspace,
            server::workspace::commands::delete_workspace,
            server::workspace::commands::get_workspace,
            server::workspace::commands::start_audio_to_text,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
