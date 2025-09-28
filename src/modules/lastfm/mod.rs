// modules/lastfm/mod.rs

// submodules
pub mod top;

// reexports
pub use top::get_top_tracks;

// structs

// track info struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Track {
    pub name: String,
    pub artist: String,
}

