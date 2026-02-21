// modules/youtube/search.rs

use crate::modules::model::Track;

/// construct youtube search query from track title and artist
pub fn get_query(track: &Track) -> String {
    let query = format!("{} {} Album Audio", track.artist, track.name);

    query
}
