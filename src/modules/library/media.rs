// modules/library/media.rs

use std::path::Path;

use id3::{Tag, TagLike, Version};
use crate::modules::lastfm::Track;

/// Apply ID3 metadata (title, artist, album, year, cover art) to an MP3 file.
pub fn apply_metadata(path: &Path, track: &Track) -> Result<(), String> {
    // Load existing tag or start new
    let mut tag = Tag::read_from_path(path).unwrap_or_else(|_| Tag::new());

    // Basic fields
    tag.set_title(&track.name);
    println!("[METADATA] Added title for {}", &track.name);
    tag.set_artist(&track.artist);
    println!("[METADATA] Added artist for {}", &track.name);

    // Write tags back to file
    tag.write_to_path(path, Version::Id3v24)
        .map_err(|e| format!("[METADATA] Failed to write ID3 tags: {}", e))?;

    Ok(())
}
