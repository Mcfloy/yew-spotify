use serde::{Deserialize, Serialize};
use crate::types::{Paging, ExternalUrl, Image};
use crate::types::user::PublicUser;
use crate::types::followers::Followers;
use crate::types::track::{PlaylistTrack, FullTrack};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: ExternalUrl,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    // tracks
    pub r#type: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlaylistFull {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: ExternalUrl,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Paging<PlaylistTrack<FullTrack>>,
    pub r#type: String,
    pub uri: String,
}