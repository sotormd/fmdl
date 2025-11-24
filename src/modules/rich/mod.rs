pub mod read;
pub mod get;
pub mod apply;

pub use read::read_basic_metadata;
pub use get::get_rich_metadata;
pub use apply::apply_rich_metadata;

#[derive(Debug)]
pub struct TrackRichInfo {
    pub album_name: Option<String>,
    pub album_position: Option<u32>,
    pub release_date: Option<String>,
    pub cover_art_url: Option<String>,
}

