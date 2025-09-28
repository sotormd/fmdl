// main.rs

mod modules;

use dotenv::dotenv;
use modules::lastfm::get_top_tracks;
use modules::library::{apply_metadata, track_filename};
use modules::youtube::{download, get_query};
use std::env;
use std::fs;
use std::fs::rename;
use std::path::Path;

use crate::modules::library::get_diff;

fn main() {
    // load environment variables
    // from .env
    dotenv().ok();

    // read api key
    let api_key = env::var("LASTFM_API_KEY").expect("[LASTFM] LASTFM_API_KEY not set");

    // read username
    let username = env::var("LASTFM_USERNAME").expect("[LASTFM] LASTFM_USERNAME not set");

    println!(
        "[LASTFM] Fetched username and API key for user {}",
        username
    );

    let cache_dir = Path::new("./.cache");
    let library_dir = Path::new("./music");

    if let Err(e) = fs::create_dir_all(cache_dir) {
        eprintln!("[LIBRARY] Failed to create cache directory: {}", e);
        return;
    }

    if let Err(e) = fs::create_dir_all(library_dir) {
        eprintln!("[LIBRARY] Failed to create library directory: {}", e);
        return;
    }

    let tracks = get_top_tracks(&username, &api_key);

    if tracks.is_empty() {
        println!("No tracks found, exiting.");
        return;
    }

    let total_tracks = tracks.len();

    println!("[LASTFM] Fetched {} tracks.", total_tracks);

    let diff = get_diff(library_dir, tracks);

    let tracks_to_add = diff.add.len();
    println!("[LIBRARY] Tracks to add    : {}", tracks_to_add);
    println!("[LIBRARY] Tracks to remove : {}", diff.remove.len());

    for file_path in diff.remove {
        if let Err(e) = fs::remove_file(&file_path) {
            eprintln!("[LIBRARY] Failed to remove {}: {}", file_path.display(), e);
        } else {
            println!("[LIBRARY] Removed {}", file_path.display());
        }
    }

    for (x, track) in diff.add.into_iter().enumerate() {
        println!();
        println!(
            "[LIBRARY] {} / {}: {} - {}",
            x + 1,
            tracks_to_add,
            track.name,
            track.artist
        );
        let query = get_query(&track);
        let cache_file_path = cache_dir.join(track_filename(&track));
        let library_file_path = library_dir.join(track_filename(&track));

        match download(&query, &cache_file_path) {
            Ok(_) => {
                println!("[LIBRARY] Downloaded {}", cache_file_path.display());

                if let Err(e) = apply_metadata(&cache_file_path, &track) {
                    eprintln!(
                        "[LIBRARY] Failed to apply metadata for {}: {}",
                        cache_file_path.display(),
                        e
                    );
                    continue; // skip moving if metadata fails
                }

                if let Err(e) = rename(&cache_file_path, &library_file_path) {
                    eprintln!(
                        "[LIBRARY] Failed to move {} to {}: {}",
                        cache_file_path.display(),
                        library_file_path.display(),
                        e
                    );
                } else {
                    println!(
                        "[LIBRARY] Moved {} to {}",
                        cache_file_path.display(),
                        library_file_path.display()
                    );
                }
            }
            Err(e) => eprintln!(
                "[LIBRARY] Failed to download {}: {}",
                cache_file_path.display(),
                e
            ),
        }
    }

    println!("Done.")
}
