// modules/library/diff.rs

use super::track_filename;
use crate::modules::lastfm::Track;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Diff {
    pub add: Vec<Track>,
    pub remove: Vec<PathBuf>,
}

pub fn get_diff(library_dir: &Path, tracks: Vec<Track>) -> Diff {
    let existing_files: Vec<PathBuf> = fs::read_dir(library_dir)
        .map(|entries| {
            entries
                .filter_map(|entrywrapper| entrywrapper.ok().map(|entry| entry.path()))
                .collect()
        })
        .unwrap_or_else(|_| vec![]);

    let mut add: Vec<Track> = Vec::new();
    let mut keep: HashSet<PathBuf> = HashSet::new();

    for track in tracks {
        // tracks is consumed here and can not be used anymore
        let track_path = library_dir.join(track_filename(&track));

        if track_path.exists() {
            // the track is already in our library
            keep.insert(track_path);
        } else {
            add.push(track); // if the track isn't there, add it
        }
    }

    let remove: Vec<PathBuf> = existing_files
        .into_iter()
        .filter(|p| !keep.contains(p)) // all files in existing_files
        // that are not contained in keep
        .collect();

    println!("[LIBRARY] Built diff");

    Diff { add, remove }
}
