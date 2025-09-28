// modules/youtube/dl.rs

use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DlError {
    #[error("[YT-DLP] failed to execute yt-dlp: {0}")]
    CommandFailed(#[from] std::io::Error),

    #[error("[YT-DLP] yt-dlp exited with status {0}")]
    ExitFailure(std::process::ExitStatus),

    #[error("[YT-DLP] invalid output path")]
    InvalidOutputPath,
}

pub fn download(query: &str, output_path: &Path) -> Result<(), DlError> {
    let output_str = match output_path.to_str() {
        Some(s) => s,
        None => return Err(DlError::InvalidOutputPath),
    };

    let mut cmd = Command::new("yt-dlp");
    
    cmd.arg("--no-playlist")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--quiet")
        .arg("--no-warnings")
        .arg("--output")
        .arg(output_str)
        .arg(format!("ytsearch1:{}", query));

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => return Err(DlError::CommandFailed(e)),
    };

    if status.success() {
        Ok(())
    } else {
        Err(DlError::ExitFailure(status))
    }
}
