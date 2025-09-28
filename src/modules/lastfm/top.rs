// modules/lastfm/top.rs

use super::Track;
use std::collections::HashSet;
use reqwest::blocking::Client;
use serde::Deserialize;

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

pub fn get_top_tracks(username: &str, api_key: &str) -> Vec<Track> {
    let client = Client::new();
    let periods = ["7day", "1month", "3month", "6month", "12month", "overall"];

    let mut all_tracks: Vec<Track> = Vec::new();

    for period in periods.iter() {
        let url = format!(
            "http://ws.audioscrobbler.com/2.0/?method=user.gettoptracks&user={}&api_key={}&format=json&period={}&limit=200",
            username, api_key, period
        );

        let resp = client.get(&url)
            .send()
            .expect("[LASTFM] Failed to fetch top tracks")
            .text()
            .expect("[LASTFM] Failed to read response");

        let json: LastfmTopTracks = serde_json::from_str(&resp)
            .expect("[LASTFM] Failed to parse JSON");

        // convert lastfm tracks to track struct
        let tracks: Vec<Track> = json.toptracks.track.into_iter().map(|t| Track {
            name: t.name,
            artist: t.artist.name,
        }).collect();

        println!("[LASTFM] Fetched {} tracks in time period {}", tracks.len(), period);

        all_tracks.extend(tracks);
    }

    let unique: HashSet<Track> = all_tracks.into_iter().collect();
    let all_tracks: Vec<Track> = unique.into_iter().collect();
    all_tracks
}
