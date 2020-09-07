use yew::prelude::*;
use crate::types::Track;
use crate::api;
use crate::api::{TopTracksResponse, FetchResponse};
use anyhow::Error;
use yew::services::fetch::FetchTask;
use yew::format::Json;
use crate::components::{TrackTableHeader, TrackTable};

pub struct TopTracks {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

struct State {
    tracks: Option<Vec<Track>>,
    get_top_tracks_error: Option<Error>,
    get_top_tracks_loaded: bool
}

#[derive(Properties, Clone)]
pub struct Props {
    pub artist_id: String
}

pub enum Msg {
    GetTopTracks,
    GetTopTracksSuccess(TopTracksResponse),
    GetTopTracksError(Error)
}

impl Component for TopTracks {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetTopTracks);
        Self {
            props,
            state: State {
              tracks: None,
                get_top_tracks_error: None,
                get_top_tracks_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetTopTracks => {
                self.state.get_top_tracks_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<TopTracksResponse>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(top_tracks_response) => Msg::GetTopTracksSuccess(top_tracks_response),
                            Err(err) => Msg::GetTopTracksError(err),
                        }
                    });

                self.task = Some(api::get_artist_top_tracks(self.props.artist_id.clone(), handler));
                true
            },
            Msg::GetTopTracksSuccess(top_track_response) => {
                self.state.get_top_tracks_loaded = true;
                self.state.tracks = Some(top_track_response.tracks);
                true
            },
            Msg::GetTopTracksError(error) => {
                self.state.get_top_tracks_loaded = true;
                self.state.get_top_tracks_error = Some(error);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
            self.link.send_message(Msg::GetTopTracks);
        true
    }

    fn view(&self) -> Html {
        let headers = vec![
            TrackTableHeader::Name,
            TrackTableHeader::Duration,
            TrackTableHeader::Popularity
        ];
        if let Some(ref tracks) = self.state.tracks {
            html! {
                <TrackTable
                    tracks=tracks
                    headers=headers
                />
            }
        } else if !self.state.get_top_tracks_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_top_tracks_error {
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