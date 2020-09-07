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
    pub disc_number: u16,
    pub duration_ms: u32,
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
pub struct Track {
    pub album: Option<SimplifiedAlbum>,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u16,
    pub duration_ms: u32,
    pub explicit: bool,
    // pub external_ids: Option<Vec<ExternalId>> ?
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    // pub linked_from: LinkedTrack,
    // pub restrictions
    pub name: String,
    pub preview_url: Option<String>,
    pub popularity: Option<u8>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool,
    pub added_at: Option<DateTime<Utc>>
}

impl Track {
    pub fn from_playlist_track(playlist_track: &PlaylistTrack<FullTrack>) -> Self {
        // TODO Remove safe cast
        let track = &playlist_track.track;
        Self {
            album: Option::from(track.album.clone()),
            artists: track.artists.clone(),
            available_markets: Some(track.available_markets.clone()),
            disc_number: track.disc_number as u16,
            duration_ms: track.duration_ms as u32,
            explicit: track.explicit,
            external_urls: track.external_urls.clone(),
            href: track.href.clone(),
            id: track.id.clone(),
            is_playable: track.is_playable.clone(),
            name: track.name.clone(),
            preview_url: track.preview_url.clone(),
            popularity: Option::from(track.popularity),
            track_number: track.track_number as u32,
            r#type: track.r#type.clone(),
            uri: track.uri.clone(),
            is_local: track.is_local,
            added_at: playlist_track.added_at
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Episode {
    // TODO
    // https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-full
}