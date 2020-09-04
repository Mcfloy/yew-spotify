use yew::format::{Json};
use yew::services::fetch::{Response};
use anyhow::Error;
use yew::callback::Callback;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

mod album_api;
mod playlist_api;
mod artist_api;

pub use album_api::*;
pub use playlist_api::*;
pub use artist_api::*;
