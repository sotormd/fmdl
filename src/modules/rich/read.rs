use id3::{Tag, TagLike};
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
pub struct TrackInfo {
    pub artist: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Error)]
pub enum ReadId3Error {
    #[error("could not read ID3 tags from file: {0}")]
    ReadError(String),

    #[error("file does not exist or could not be accessed: {0}")]
    FileAccess(String),
}

pub fn read_basic_metadata<P: AsRef<Path>>(path: P) -> Result<TrackInfo, ReadId3Error> {
    let path_ref = path.as_ref();

    // Check if the file exists and is accessible
    match std::fs::metadata(path_ref) {
        Ok(_) => {}
        Err(e) => {
            return Err(ReadId3Error::FileAccess(format!(
                "{} ({})",
                path_ref.display(),
                e
            )))
        }
    }

    // Try reading the ID3 tag
    let tag = match Tag::read_from_path(path_ref) {
        Ok(tag) => tag,
        Err(e) => {
            return Err(ReadId3Error::ReadError(format!(
                "{} ({})",
                path_ref.display(),
                e
            )))
        }
    };

    // Extract artist and title safely
    let artist = tag.artist().map(|s| s.to_owned());
    let title = tag.title().map(|s| s.to_owned());

    Ok(TrackInfo { artist, title })
}
