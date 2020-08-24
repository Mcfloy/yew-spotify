use yew::prelude::*;
use anyhow::Error;
use crate::types::PlaylistFull;
use yew::services::fetch::FetchTask;
use crate::api;
use crate::api::FetchResponse;
use yew::format::Json;

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
    GetPlaylistError(Error)
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
                self.state.playlist = Some(playlist);
                true
            }
            Msg::GetPlaylistError(error) => {
                self.state.get_playlist_loaded = true;
                self.state.get_playlist_error = Some(error);
                true
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
                .map(|ref playlist_track| {
                    let track = &playlist_track.track;
                    let artists: String = track.artists
                        .iter()
                        .map(|a| a.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ");
                    html! {
                        <tr>
                            <td>{&track.name}</td>
                            <td>{artists}</td>
                            <td>{&track.album.name}</td>
                            <td>{&playlist_track.added_at.unwrap().format("%F")}</td>
                            <td>{&track.popularity}</td>
                            <td>{&track.duration_ms}</td>
                        </tr>
                    }
                })
                .collect();
            html! {
                <>
                    <h3>{&playlist.name}</h3>
                    <table>
                        <tbody>
                            { tracks }
                        </tbody>
                    </table>
                </>
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