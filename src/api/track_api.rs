use crate::utils::{get_access_token};
use crate::types::{Paging, FullTrack, PlaylistTrack};
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::api::FetchCallback;

pub fn get_tracks_from_uri(request_uri: &str, callback: FetchCallback<Paging<PlaylistTrack<FullTrack>>>) -> FetchTask {
    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}