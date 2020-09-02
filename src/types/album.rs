use serde::{Deserialize, Serialize};
use crate::types::{ExternalUrl, Image, Paging, SimplifiedTrack, Track};
use crate::types::artist::SimplifiedArtist;
use crate::types::copyright::Copyright;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimplifiedAlbum {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    // pub restrictions
    pub r#type: String,
    pub uri: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FullAlbum {
    pub album_type: String,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    // pub external_ids: ExternalId,
    pub external_urls: ExternalUrl,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub label: String,
    pub name: String,
    pub popularity: i32,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: Paging<Track>,
    pub r#type: String,
    pub uri: String
}

