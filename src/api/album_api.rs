use crate::utils::get_access_token;
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::types::FullAlbum;
use crate::api::FetchCallback;

pub fn get_album(album_id: String, callback: FetchCallback<FullAlbum>) -> FetchTask {
    let request_uri = format!("https://api.spotify.com/v1/albums/{}", &album_id);

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}