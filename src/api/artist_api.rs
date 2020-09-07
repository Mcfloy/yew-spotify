use crate::utils::{get_access_token, getItem};
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::api::FetchCallback;
use serde::{Deserialize, Serialize};
use crate::types::{Artist, SimplifiedAlbum, Paging, Track, PrivateUser};

pub fn get_artist(artist_id: String, callback: FetchCallback<Artist>) -> FetchTask {
    let request_uri = format!("https://api.spotify.com/v1/artists/{}", artist_id);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_artist_albums(artist_id: String, include_groups: Vec<String>, limit: Option<u8>, offset: Option<u8>, callback: FetchCallback<Paging<SimplifiedAlbum>>) -> FetchTask {
    let user_str = getItem(String::from("user"));
    let user = serde_json::from_str::<PrivateUser>(user_str.as_str()).unwrap();

    let include_groups_param = if include_groups.is_empty() {
        String::from("")
    } else {
        format!("&include_groups={}", include_groups.join(","))
    };
    let limit_param = if let Some(limit) = limit {
        format!("&limit={}", limit)
    } else {
        String::from("")
    };
    let offset_param = if let Some(offset) = offset {
        format!("&offset={}", offset)
    } else {
        String::from("")
    };

    let request_uri = format!("https://api.spotify.com/v1/artists/{}/albums?country={}{}{}{}", artist_id, user.country, include_groups_param, limit_param, offset_param);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TopTracksResponse {
    pub tracks: Vec<Track>
}

pub fn get_artist_top_tracks(artist_id: String, callback: FetchCallback<TopTracksResponse>) -> FetchTask {
    let user_str = getItem(String::from("user"));
    let user = serde_json::from_str::<PrivateUser>(user_str.as_str()).unwrap();
    let request_uri = format!("https://api.spotify.com/v1/artists/{}/top-tracks?country={}", artist_id, user.country);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}