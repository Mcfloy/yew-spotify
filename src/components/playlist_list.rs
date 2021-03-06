use yew::prelude::*;
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use crate::api::FetchResponse;
use crate::api;
use crate::types::{Paging, SimplifiedPlaylist};
use crate::components::playlist_item::PlaylistItem;

struct State {
    playlists: Vec<SimplifiedPlaylist>,
    get_playlists_error: Option<Error>,
    get_playlists_loaded: bool
}

pub struct PlaylistList {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

pub enum Msg {
    GetPlaylists(Option<String>),
    GetPlaylistsSuccess(Vec<SimplifiedPlaylist>, Option<String>),
    GetPlaylistsError(Error)
}

impl Component for PlaylistList {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let playlists = vec![];

        link.send_message(Msg::GetPlaylists(None));

        Self {
            state: State {
                playlists,
                get_playlists_error: None,
                get_playlists_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPlaylists(option_request_uri) => {
                self.state.get_playlists_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: FetchResponse<Paging<SimplifiedPlaylist>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(paging) => Msg::GetPlaylistsSuccess(paging.items, paging.next),
                                Err(err) => Msg::GetPlaylistsError(err)
                            }
                        });
                match option_request_uri {
                    Some(request_uri) => {
                        self.task = Some(api::get_playlists_from_uri(&request_uri, handler));
                    },
                    None => {
                        self.task = Some(api::get_me_playlists(handler));
                    }
                }
                true
            }
            Msg::GetPlaylistsSuccess(mut playlists, option_request_uri) => {
                self.state.playlists.append(&mut playlists);

                match option_request_uri {
                    Some(request_uri) => {
                        self.link.send_message(Msg::GetPlaylists(Some(request_uri)));
                    },
                    None => {
                        self.state.get_playlists_loaded = true;
                    }
                }
                true
            }
            Msg::GetPlaylistsError(error) => {
                self.state.get_playlists_error = Some(error);
                self.state.get_playlists_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let playlists: Vec<Html> = self.state
            .playlists
            .iter()
            .map(|playlist| {
                html! {
                    <PlaylistItem id={&playlist.id} name={&playlist.name} />
                }
            })
            .collect();

        if !self.state.get_playlists_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_playlists_error {
            html! {
                <div>
                    <span>{"Error loading playlists :("} {error}</span>
                </div>
            }
        } else {
            html! {
                <div>
                    <h3>{"Playlists"}</h3>
                    <ul id="my_playlists">{playlists}</ul>
                </div>
            }
        }
    }
}