use crate::utils::get_access_token;
use crate::types::{Paging, SimplifiedPlaylist, PlaylistFull};
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::api::FetchCallback;


pub fn get_me_playlists(callback: FetchCallback<Paging<SimplifiedPlaylist>>) -> FetchTask {
    let request_uri = "https://api.spotify.com/v1/me/playlists?limit=50";
    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_playlist(playlist_id: String, callback: FetchCallback<PlaylistFull>) -> FetchTask {
    let request_uri = format!("{}/{}?limit=50", "https://api.spotify.com/v1/playlists", playlist_id);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}



pub fn get_playlists_from_uri(request_uri: &str, callback: FetchCallback<Paging<SimplifiedPlaylist>>) -> FetchTask {
    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}