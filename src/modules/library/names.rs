// modules/library/names.rs

use crate::modules::lastfm::Track;

fn sanitize_filename(s: &str) -> String {
    let forbidden = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    s.chars()
        .map(|c| if forbidden.contains(&c) { '_' } else { c })
        .collect()
}

pub fn track_filename(track: &Track) -> String {
    let parts = format!("{} - {}", track.name, track.artist);

    sanitize_filename(&parts) + ".mp3"
}
