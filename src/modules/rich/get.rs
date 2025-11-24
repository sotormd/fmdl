use reqwest::blocking::Client;
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;
use super::TrackRichInfo;

#[derive(Debug, Error)]
pub enum LastFmError {
    #[error("HTTP request failed: {0}")]
    Http(String),

    #[error("Invalid response format: {0}")]
    Parse(String),

    #[error("API error: code {code}, message {message}")]
    Api { code: u32, message: String },

    #[error("Missing required fields in response")]
    MissingFields,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    error: u32,
    message: String,
}

#[derive(Deserialize)]
struct TrackGetInfoResponse {
    track: Option<TrackNode>,
}

#[derive(Deserialize)]
struct TrackNode {
    album: Option<AlbumNode>,
    wiki: Option<WikiNode>,
}

#[derive(Deserialize)]
struct AlbumNode {
    #[serde(rename = "title")]
    album_name: Option<String>,
    #[serde(rename = "artist")]
    _album_artist: Option<String>, // maybe not needed
    #[serde(rename = "position")]
    position: Option<String>,
    #[serde(rename = "image")]
    image: Option<Vec<ImageNode>>,
}

#[derive(Deserialize)]
struct WikiNode {
    published: Option<String>,
}

#[derive(Deserialize)]
struct ImageNode {
    #[serde(rename = "size")]
    size: Option<String>,
    #[serde(rename = "#text")]
    url: Option<String>,
}

pub fn get_rich_metadata(
    api_key: &str,
    artist: &str,
    title: &str,
) -> Result<TrackRichInfo, LastFmError> {
    let url = format!(
        "http://ws.audioscrobbler.com/2.0/?method=track.getInfo&api_key={}&artist={}&track={}&format=json",
        api_key,
        urlencoding::encode(artist),
        urlencoding::encode(title)
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| LastFmError::Http(e.to_string()))?;

    let resp = client
        .get(&url)
        .send()
        .map_err(|e| LastFmError::Http(e.to_string()))?;
    
    let status = resp.status();
    let text = resp
        .text()
        .map_err(|e| LastFmError::Http(e.to_string()))?;

    if !status.is_success() {
        // Attempt to parse error body
        if let Ok(err_resp) = serde_json::from_str::<ApiErrorResponse>(&text) {
            return Err(LastFmError::Api { code: err_resp.error, message: err_resp.message });
        } else {
            return Err(LastFmError::Http(format!("Status {}: {}", status, text)));
        }
    }

    let parsed: TrackGetInfoResponse =
        serde_json::from_str(&text).map_err(|e| LastFmError::Parse(e.to_string()))?;

    let track_node = parsed.track.ok_or(LastFmError::MissingFields)?;

    // Album info
    let (album_name, album_position, cover_art_url) = if let Some(album) = track_node.album {
        // album name
        let name = album.album_name.clone();

        // position string -> parse to u32
        let pos = album.position.and_then(|s| s.parse::<u32>().ok());

        // pick an image: choose largest size if available
        let url_opt = album.image.and_then(|images| {
            // find image node with size "large" or else last in vector
            images.iter().rev().find_map(|img| img.url.clone())
        });

        (name, pos, url_opt)
    } else {
        (None, None, None)
    };

    // Release date from wiki node
    let release_date = track_node.wiki.and_then(|w| w.published);

    Ok(TrackRichInfo {
        album_name,
        album_position,
        release_date,
        cover_art_url,
    })
}
