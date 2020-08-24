use serde::{Deserialize, Serialize};
use crate::types::{ExternalUrl, Image};
use crate::types::artist::SimplifiedArtist;

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