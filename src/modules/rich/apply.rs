use id3::{Tag, TagLike, Version, frame::Picture, frame::PictureType};
use std::path::Path;
use std::fmt;
use super::TrackRichInfo;

#[derive(Debug)]
pub enum ApplyError {
    Read(String),
    Write(String),
    CoverArtDownload(String),
    NoAlbumName,
}

/// User-friendly Display implementation
impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::Read(msg) => write!(f, "Failed to read file: {}", msg),
            ApplyError::Write(msg) => write!(f, "Failed to write tags: {}", msg),
            ApplyError::CoverArtDownload(msg) => write!(f, "Failed to download cover art: {}", msg),
            ApplyError::NoAlbumName => write!(f, "Missing album name in metadata"),
        }
    }
}

/// Implement std::error::Error so ApplyError composes with other errors
impl std::error::Error for ApplyError {}

/// Optional: convenient conversions from common error types
impl From<std::io::Error> for ApplyError {
    fn from(e: std::io::Error) -> Self {
        ApplyError::Read(e.to_string())
    }
}

impl From<id3::Error> for ApplyError {
    fn from(e: id3::Error) -> Self {
        // id3::Error can represent read/write errors; map to Write for simplicity
        ApplyError::Write(e.to_string())
    }
}

impl From<reqwest::Error> for ApplyError {
    fn from(e: reqwest::Error) -> Self {
        ApplyError::CoverArtDownload(e.to_string())
    }
}

pub fn apply_rich_metadata<P: AsRef<Path>>(
    info: &TrackRichInfo,
    filepath: P,
) -> Result<(), ApplyError> {
    let path_ref = filepath.as_ref();

    // --- Load or create tag ---
    let mut tag = match Tag::read_from_path(path_ref) {
        Ok(tag) => tag,
        Err(_) => Tag::new(),
    };

    // --- Album name ---
    if let Some(album_name) = &info.album_name {
        tag.set_album(album_name.clone());
    }

    // --- Track number within album ---
    if let Some(pos) = info.album_position {
        tag.set_track(pos);
    }

    // --- Release year (extract from date string) ---
    if let Some(date_str) = &info.release_date {
        if let Some(year) = extract_year_from_date(date_str) {
            tag.set_year(year as i32);
        }
    }

    // --- Cover art ---
    if let Some(url) = &info.cover_art_url {
        match download_cover_art(url) {
            Ok(bytes) => {
                let picture = Picture {
                    mime_type: guess_mime_type(url),
                    picture_type: PictureType::CoverFront,
                    description: "Cover".to_string(),
                    data: bytes,
                };
                tag.add_frame(picture);
            }
            Err(e) => return Err(ApplyError::CoverArtDownload(e)),
        }
    }

    // --- Save tag ---
    match tag.write_to_path(path_ref, Version::Id3v24) {
        Ok(_) => Ok(()),
        Err(e) => Err(ApplyError::Write(e.to_string())),
    }
}

fn extract_year_from_date(date: &str) -> Option<u32> {
    // Last.fm published: "Sun, 27 Jul 2008 15:44:58 +0000"
    // Extract last 4 digits
    date.trim().rsplit(' ').find_map(|p| p.parse::<u32>().ok())
}

fn guess_mime_type(url: &str) -> String {
    if url.ends_with(".png") { "image/png".into() }
    else { "image/jpeg".into() }
}

fn download_cover_art(url: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    resp.bytes()
        .map(|b| b.to_vec())
        .map_err(|e| e.to_string())
}
