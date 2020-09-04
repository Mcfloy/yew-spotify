use serde::{Deserialize, Serialize};
use crate::types::{ExternalUrl, Image};
use crate::types::followers::Followers;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Artist {
    pub external_urls: ExternalUrl,
    pub followers: Option<Followers>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub popularity: Option<u8>,
    pub r#type: String,
    pub uri: String,
}