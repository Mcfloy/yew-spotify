use crate::utils::get_access_token;
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::api::FetchCallback;
use serde::{Deserialize, Serialize};
use crate::types::{Artist, SimplifiedAlbum, Paging, Track};

pub fn get_artist(artist_id: String, callback: FetchCallback<Artist>) -> FetchTask {
    let request_uri = format!("https://api.spotify.com/v1/artists/{}", artist_id);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_artist_albums(artist_id: String, callback: FetchCallback<Paging<SimplifiedAlbum>>) -> FetchTask {
    let request_uri = format!("https://api.spotify.com/v1/artists/{}/albums", artist_id);

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
    let request_uri = format!("https://api.spotify.com/v1/artists/{}/top-tracks", artist_id);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}