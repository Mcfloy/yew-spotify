use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::user::PublicUser;
use crate::types::ExternalUrl;
use crate::types::album::SimplifiedAlbum;
use crate::types::artist::SimplifiedArtist;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlaylistTrack<T> {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    pub track: T
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    // pub external_ids
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    // pub linked_from: LinkedTrack,
    // pub restriction
    pub name: String,
    pub popularity: u8,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    // pub linked_from: LinkedTrack,
    // pub restrictions
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Episode {
    // TODO
    // https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-full
}