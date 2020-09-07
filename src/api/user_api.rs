use crate::utils::get_access_token;
use yew::services::fetch::{FetchService, FetchTask, Request};
use yew::format::Nothing;
use crate::api::FetchCallback;
use serde::{Deserialize, Serialize};
use crate::types::PrivateUser;

pub fn get_current_user_profile(callback: FetchCallback<PrivateUser>) -> FetchTask {
    let request_uri = format!("https://api.spotify.com/v1/me");

    let req = Request::get(request_uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer  {}", get_access_token()))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}