use yew::prelude::*;
use crate::types::FullAlbum;
use anyhow::Error;
use yew::{ComponentLink, Component, Html};
use yew::services::fetch::{FetchTask};
use crate::api::FetchResponse;
use crate::api;
use yew::format::Json;
use crate::components::{AppHeader, TrackTable, TrackTableHeader};

struct State {
    album: Option<FullAlbum>,
    get_album_error: Option<Error>,
    get_album_loaded: bool
}

pub struct AlbumDetail {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: String,
}

pub enum Msg {
    GetAlbum,
    GetAlbumSuccess(FullAlbum),
    GetAlbumError(Error)
}

impl Component for AlbumDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetAlbum);

        Self {
            props,
            state: State {
                album: None,
                get_album_error: None,
                get_album_loaded: false
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetAlbum => {
                self.state.get_album_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<FullAlbum>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(album) => Msg::GetAlbumSuccess(album),
                            Err(err) => Msg::GetAlbumError(err),
                        }
                    });

                self.task = Some(api::get_album(self.props.id.clone(), handler));
                true
            },
            Msg::GetAlbumSuccess(album) => {
                self.state.get_album_loaded = true;
                self.state.album = Some(album);
                true
            },
            Msg::GetAlbumError(error) => {
                self.state.get_album_loaded = true;
                self.state.get_album_error = Some(error);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.link.send_message(Msg::GetAlbum);
        true
    }

    fn view(&self) -> Html {
        if let Some(ref album) = self.state.album {
            let headers = vec![
                TrackTableHeader::TrackNumber,
                TrackTableHeader::Name,
                TrackTableHeader::Duration
            ];

            let total_duration: u32 = album.tracks.items.iter().map(|track| track.duration_ms).sum();
            html! {
                <>
                    <AppHeader
                        header_type=&album.r#type
                        name=&album.name
                        authors=&album.artists
                        image=&album.images[0].url
                        total=album.tracks.total
                        total_duration=total_duration
                        release_date=&album.release_date
                         />
                    <TrackTable tracks=&album.tracks.items headers=headers />
                </>
            }
        } else if !self.state.get_album_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_album_error {
            html! {
                <div>
                    <span>{"Error:"} {error}</span>
                </div>
            }
        } else {
            html! {
                <div>{"Unknown state"}</div>
            }
        }
    }
}