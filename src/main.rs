// main.rs

mod modules;

use clap::Parser;
use dotenv::dotenv;
use modules::lastfm::get_top_tracks;
use modules::library::{apply_metadata, get_diff, track_filename};
use modules::youtube::{download, get_query};
use std::env;
use std::fs;
use std::fs::copy;
use std::path::Path;

/// Manage a local library based on LastFM top tracks.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Sync local library with top tracks
    #[arg(short, long)]
    sync: bool,

    /// Show diff between local library and top tracks
    #[arg(short, long)]
    diff: bool,

    /// Do not remove tracks marked for removal while syncing
    #[arg(short = 'k', long)]
    keep_all: bool,

    /// Do not print excess messages
    #[arg(short = 'q', long)]
    shut_up: bool,

    /// Path to save tracks
    #[arg(short = 'l', long, default_value = "./music")]
    library_path: String,

    /// Path to download tracks
    #[arg(short = 'c', long, default_value = "./.cache")]
    cache_path: String,
}

fn main() {
    // load environment variables
    // from .env
    dotenv().ok();

    // parse cli arguments
    let args = Args::parse();

    // neither --sync nor --diff mentioned
    if !args.sync && !args.diff {
        sync(&args);
    }
    // --diff is mentioned
    else if args.diff {
        diff(&args);
    }
    // --sync is mentioned
    else if args.sync {
        sync(&args);
    }
}

/// Sync local library with top tracks
fn sync(args: &Args) {
    let username = env::var("LASTFM_USERNAME").expect("[LASTFM] LASTFM_USERNAME not set.");
    let api_key = env::var("LASTFM_API_KEY").expect("[LASTFM] LASTFM_API_KEY not set.");

    if !args.shut_up {
        println!(
            "[LASTFM] Fetched username and API key for user {}",
            username
        );
    }

    let cache_dir = Path::new(&args.cache_path);
    let library_dir = Path::new(&args.library_path);

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
        println!("[LASTFM] No tracks found, exiting.");
        return;
    }

    let diff = get_diff(library_dir, tracks);

    let add_len = diff.add.len();
    let remove_len = diff.remove.len();

    if !args.shut_up {
        println!("[LIBRARY] Tracks to add    : {}", add_len);
        println!("[LIBRARY] Tracks to remove : {}", remove_len);
    }

    if !args.keep_all {
        for file_path in diff.remove {
            if let Err(e) = fs::remove_file(&file_path) {
                eprintln!(
                    "[LIBRARY] Failed to remove \"{}\": {}",
                    file_path.display(),
                    e
                );
            } else if !args.shut_up {
                println!("[LIBRARY] Removed \"{}\"", file_path.display());
            }
        }
    }

    for (x, track) in diff.add.into_iter().enumerate() {
        if !args.shut_up {
            println!(
                "[LIBRARY] {} / {}: {} - {}",
                x + 1,
                add_len,
                track.name,
                track.artist
            );
        }

        let query = get_query(&track);
        let cache_file_path = cache_dir.join(track_filename(&track));
        let library_file_path = library_dir.join(track_filename(&track));

        match download(&query, &cache_file_path) {
            Ok(_) => {
                if let Err(e) = apply_metadata(&cache_file_path, &track) {
                    eprintln!(
                        "[LIBRARY] Failed to apply metadata for {}: {}",
                        cache_file_path.display(),
                        e
                    );
                    continue;
                }

                if let Err(e) = copy(&cache_file_path, &library_file_path) {
                    eprintln!(
                        "[LIBRARY] Failed to move \"{}\" to \"{}\": {}",
                        cache_file_path.display(),
                        library_file_path.display(),
                        e
                    );
                } else if !args.shut_up {
                    println!(
                        "[LIBRARY] Saved \"{}\"",
                        library_file_path.display()
                    );
                }
            }
            Err(e) => eprintln!(
                "[LIBRARY] Failed to download \"{}\": {}",
                cache_file_path.display(),
                e
            ),
        }
    }

    if let Err(e) = fs::remove_dir_all(&cache_dir) {
        eprintln!("[LIBRARY] Failed to delete cache directory");
        return;
    }

    if !args.shut_up {
        println!("Done.");
    }
}

/// Show diff between local library and top tracks
fn diff(args: &Args) {
    let username = env::var("LASTFM_USERNAME").expect("[LASTFM] LASTFM_USERNAME not set.");
    let api_key = env::var("LASTFM_API_KEY").expect("[LASTFM] LASTFM_API_KEY not set.");

    if !args.shut_up {
        println!(
            "[LASTFM] Fetched username and API key for user {}",
            username
        );
    }

    let library_dir = Path::new(&args.library_path);
    let tracks = get_top_tracks(&username, &api_key);

    if tracks.is_empty() {
        println!("[LASTFM] No tracks found, exiting.");
        return;
    }

    let diff = get_diff(library_dir, tracks);

    println!("[LIBRARY] Tracks to add:");
    for (x, track) in diff.add.into_iter().enumerate() {
        println!("{}.  {} - {}", x + 1, track.name, track.artist);
    }

    println!();
    println!("[LIBRARY] Files to remove:");
    for (x, file) in diff.remove.into_iter().enumerate() {
        println!("{}.  {}", x + 1, file.display());
    }
}

