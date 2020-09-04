use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/playlist/{id}"]
    PlaylistDetail(String),
    #[to = "/album/{id}"]
    AlbumDetail(String),
    #[to = "/artist/{id}"]
    ArtistDetail(String),
    #[to = "/"]
    HomePage,
}