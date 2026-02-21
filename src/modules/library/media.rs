// modules/library/media.rs

use crate::modules::model::Track;

use id3::{Tag, TagLike, Version};
use std::path::Path;

// error type for apply_metadata
#[derive(Debug)]
pub enum MetadataError {
    Write(id3::Error),
}

/// apply ID3 metadata to a mp3 file
pub fn apply_metadata(path: &Path, track: &Track) -> Result<(), MetadataError> {
    // load existing tag or start new
    let mut tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(_) => Tag::new(),
    };

    // basic fields: title and artist
    tag.set_title(&track.name);
    tag.set_artist(&track.artist);

    // write tags back to file
    match tag.write_to_path(path, Version::Id3v24) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(MetadataError::Write(e)),
    }
}
