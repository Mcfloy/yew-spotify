use yew::prelude::*;
use anyhow::Error;
use crate::types::{PlaylistFull, PlaylistTrack, FullTrack};
use yew::services::fetch::FetchTask;
use crate::api::FetchResponse;
use crate::api;
use yew::format::Json;
use crate::utils::parse_time_to_string;
use crate::utils::play;
use yew_router::prelude::RouterAnchor;
use crate::route::Route;

struct State {
    playlist: Option<PlaylistFull>,
    get_playlist_error: Option<Error>,
    get_playlist_loaded: bool
}

pub struct PlaylistDetail {
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
    GetPlaylist,
    GetPlaylistSuccess(PlaylistFull),
    GetPlaylistError(Error),
    PlayTrack(String)
}

impl Component for PlaylistDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPlaylist);

        Self {
            props,
            state: State {
                playlist: None,
                get_playlist_error: None,
                get_playlist_loaded: false
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPlaylist => {
                self.state.get_playlist_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<PlaylistFull>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(playlist) => Msg::GetPlaylistSuccess(playlist),
                            Err(err) => Msg::GetPlaylistError(err),
                        }
                    });

                self.task = Some(api::get_playlist(self.props.id.clone(), handler));
                true
            }
            Msg::GetPlaylistSuccess(playlist) => {
                self.state.get_playlist_loaded = true;
                self.state.playlist = Some(playlist);
                true
            }
            Msg::GetPlaylistError(error) => {
                self.state.get_playlist_loaded = true;
                self.state.get_playlist_error = Some(error);
                true
            }
            Msg::PlayTrack(spotify_uri) => {
                play(spotify_uri);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.link.send_message(Msg::GetPlaylist);
        true
    }

    fn view(&self) -> Html {
        if let Some (ref playlist) = self.state.playlist {
            let tracks: Vec<Html> = playlist.tracks.items
                .iter()
                .map(|playlist_track| self.view_playlist_track(playlist_track))
                .collect();

            let display_name ;
            display_name = match &playlist.owner.display_name {
                Some(unwrapped_display_name) => unwrapped_display_name.clone(),
                _ => String::from("")
            };

            html! {
                <div id="playlist_list_container">
                    <div id="playlist_list_header">
                        <div id="playlist_list_image">
                            <img src={&playlist.images[0].url} />
                        </div>
                        <h5>{"PLAYLIST"}</h5>
                        <h1>{&playlist.name}</h1>
                        <span>{"Créée par "}<b>{display_name}</b>{" • "}{&playlist.tracks.total}{" titres"}</span>
                    </div>
                    <table>
                        <thead>
                            <tr>
                                <th></th>
                                <th></th>
                                <th>{"Titre"}</th>
                                <th>{"Artiste"}</th>
                                <th>{"Album"}</th>
                                <th>{"Date"}</th>
                                <th></th>
                                <th>{"Durée"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            { tracks }
                        </tbody>
                    </table>
                </div>
            }
        }
        else if !self.state.get_playlist_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_playlist_error {
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

impl PlaylistDetail {
    fn view_playlist_track(&self, playlist_track: &PlaylistTrack<FullTrack>) -> Html {
        type Anchor = RouterAnchor<Route>;

        let track = &playlist_track.track;
        let onclick = {
            let uri = track.uri.clone();
            self.link.callback(move |_| Msg::PlayTrack(uri.clone()))
        };
        let artists: String = track.artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        html! {
            <tr>
                <td>
                    <button onclick=onclick>
                        <span class="material-icons">
                            {"play_arrow"}
                        </span>
                    </button>
                </td>
                <td></td>
                <td>{&track.name}</td>
                <td>{artists}</td>
                <td>
                    <Anchor route=Route::AlbumDetail(track.album.id.clone()) classes="playlist_item_anchor">
                        {&track.album.name}
                    </Anchor>
                </td>
                <td>{&playlist_track.added_at.unwrap().format("%F")}</td>
                <td></td>
                <td>{parse_time_to_string(&track.duration_ms)}</td>
            </tr>
        }
    }
}