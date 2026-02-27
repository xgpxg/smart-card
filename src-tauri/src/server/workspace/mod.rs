use anyhow::bail;
use common::resources_dir;
use std::process::Command;

pub mod commands;
mod service;

enum FileType {
    Audio,
    Video,
}
pub fn check_is_audio_or_video<P: Into<String>>(path: P) -> anyhow::Result<FileType> {
    let path = path.into();
    let ext = path.split(".").last().unwrap();
    match ext {
        "mp3" | "wav" | "ogg" | "m4a" | "aac" => Ok(FileType::Audio),
        "mp4" | "mov" | "avi" => Ok(FileType::Video),
        _ => {
            bail!("不支持的文件类型");
        }
    }
}

pub fn extract_audio_from_video(video_path: &str, output_path: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    let command = resources_dir!("bin", "windows", "ffmpeg", "bin", "ffmpeg.exe");
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let command = resources_dir!("bin", "ubuntu", "ffmpeg", "bin", "ffmpeg");
    let output = Command::new(command)
        .args(["-i", video_path, "-q:a", "0", "-map", "a", output_path])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("ffmpeg 执行失败: {}", error_msg));
    }

    Ok(())
}
