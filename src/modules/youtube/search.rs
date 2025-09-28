// modules/youtube/search.rs

use crate::modules::lastfm::Track;

pub fn get_query(track: &Track) -> String {
    let query = format!("{} {} Album Audio", track.artist, track.name);

    query
}
