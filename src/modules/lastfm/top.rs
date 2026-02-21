// modules/lastfm/top.rs

use crate::modules::model::Track;

use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;

// for json deserialization
#[derive(Debug, Deserialize)]
struct LastfmTopTracks {
    toptracks: Toptracks,
}

#[derive(Debug, Deserialize)]
struct Toptracks {
    track: Vec<LastfmTrack>,
}

#[derive(Debug, Deserialize)]
struct LastfmTrack {
    name: String,
    artist: Artist,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

// error type for get_top_tracks
#[derive(Debug)]
pub enum GetTopTracksError {
    Request(reqwest::Error),
    Json(serde_json::Error),
}

pub fn get_top_tracks(username: &str, api_key: &str) -> Result<Vec<Track>, GetTopTracksError> {
    let client = Client::new();
    let periods = ["7day", "1month", "3month", "6month", "12month", "overall"];

    let mut all_tracks: Vec<Track> = Vec::new();

    for period in periods.iter() {
        let url = format!(
            "http://ws.audioscrobbler.com/2.0/?method=user.gettoptracks&user={}&api_key={}&format=json&period={}&limit=200",
            username, api_key, period
        );

        let resp = match client.get(&url).send() {
            Ok(r) => r,
            Err(e) => return Err(GetTopTracksError::Request(e)),
        };

        let resp_text = match resp.text() {
            Ok(r) => r,
            Err(e) => return Err(GetTopTracksError::Request(e)),
        };

        let json: LastfmTopTracks = match serde_json::from_str(&resp_text) {
            Ok(j) => j,
            Err(e) => return Err(GetTopTracksError::Json(e)),
        };

        // convert lastfm tracks to track struct
        let tracks: Vec<Track> = json
            .toptracks
            .track
            .into_iter()
            .map(|t| Track {
                name: t.name,
                artist: t.artist.name,
            })
            .collect();

        all_tracks.extend(tracks);
    }

    let unique: HashSet<Track> = all_tracks.into_iter().collect();
    let all_tracks: Vec<Track> = unique.into_iter().collect();
    Ok(all_tracks)
}
