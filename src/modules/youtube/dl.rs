// modules/youtube/dl.rs

use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum DlError {
    CommandFailed(std::io::Error),
    ExitFailure(std::process::ExitStatus),
    InvalidOutputPath,
}

/// download track from youtube using yt-dlp
pub fn download(query: &str, output_path: &Path) -> Result<(), DlError> {
    // android probably needs utf8 paths
    let output_str = match output_path.to_str() {
        Some(s) => s,
        None => return Err(DlError::InvalidOutputPath),
    };

    let mut cmd = Command::new("yt-dlp");

    // these args seem to work, no issues till now
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
