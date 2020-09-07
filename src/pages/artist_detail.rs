use yew::prelude::*;
use anyhow::Error;
use yew::{ComponentLink, Component, Html};
use yew::services::fetch::{FetchTask};
use crate::api;
use crate::api::FetchResponse;
use yew::format::{Json};
use crate::types::Artist;
use crate::components::{AppBanner};

struct State {
    artist: Option<Artist>,
    get_artist_error: Option<Error>,
    get_artist_loaded: bool
}

pub struct ArtistDetail {
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
    GetArtist,
    GetArtistSuccess(Artist),
    GetArtistError(Error)
}

impl Component for ArtistDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetArtist);

        Self {
            props,
            state: State {
                artist: None,
                get_artist_error: None,
                get_artist_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetArtist => {
                self.state.get_artist_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<Artist>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(artist) => Msg::GetArtistSuccess(artist),
                            Err(err) => Msg::GetArtistError(err),
                        }
                    });

                self.task = Some(api::get_artist(self.props.id.clone(), handler));
                true
            },
            Msg::GetArtistSuccess(artist) => {
                self.state.get_artist_loaded = true;
                self.state.artist = Some(artist);
                true
            },
            Msg::GetArtistError(error) => {
                self.state.get_artist_loaded = true;
                self.state.get_artist_error = Some(error);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.link.send_message(Msg::GetArtist);
        true
    }

    fn view(&self) -> Html {
        if let Some(ref artist) = self.state.artist {
            html! {
                <>
                    <AppBanner artist=artist.clone() />
                    // Top tracks
                    // Albums
                    // Related artists
                </>
            }
        } else if !self.state.get_artist_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_artist_error {
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