use crate::utils::get_access_token;
use crate::types::{Paging, SimplifiedPlaylist, PlaylistFull};
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_me_playlists(callback: FetchCallback<Paging<SimplifiedPlaylist>>) -> FetchTask {
    let request_uri = "https://api.spotify.com/v1/me/playlists";
    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &get_access_token())
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_playlist(playlist_id: String, callback: FetchCallback<PlaylistFull>) -> FetchTask {
    let request_uri = "https://api.spotify.com/v1/playlists/".to_owned() + &playlist_id;

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &get_access_token())
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}