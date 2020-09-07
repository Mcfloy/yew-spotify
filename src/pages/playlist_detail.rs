use yew::prelude::*;
use anyhow::Error;
use crate::types::{PlaylistFull, Track, Artist};
use yew::services::fetch::FetchTask;
use crate::api::FetchResponse;
use crate::api;
use yew::format::Json;
use crate::components::{TrackTableHeader, TrackTable, AppHeader};

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
                self.state.get_playlist_loaded = true;
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
            let tracks: Vec<Track> = playlist.tracks.items
                .iter()
                .map(|playlist_track| Track::from_playlist_track(playlist_track))
                .collect();

            let total_duration: u32 = playlist.tracks.items
                .iter()
                .map(|playlist_track| playlist_track.track.duration_ms)
                .sum();

            let converted_owner: Vec<Artist> = match &playlist.owner.display_name {
                Some(unwrapped_display_name) => {
                    vec![Artist {
                        external_urls: playlist.owner.external_urls.clone(),
                        followers: playlist.owner.followers.clone(),
                        genres: None,
                        href: playlist.owner.href.clone(),
                        id: playlist.owner.id.clone(),
                        images: playlist.owner.images.clone(),
                        name: unwrapped_display_name.clone(),
                        popularity: None,
                        r#type: playlist.owner.r#type.clone(),
                        uri: playlist.owner.uri.clone()
                    }]
                },
                _ => {
                    vec![]
                }
            };

            let headers = vec![
                TrackTableHeader::Name,
                TrackTableHeader::Artist,
                TrackTableHeader::Album,
                TrackTableHeader::AddedAt,
                TrackTableHeader::Duration
            ];

            let release_date: Option<String> = None;

            // <th>{"Titre"}</th>
            //     <th>{"Artiste"}</th>
            //     <th>{"Album"}</th>
            //     <th>{"Date"}</th>
            //     <th></th>
            //     <th>{"Durée"}</th>

            html! {
                <>
                    <AppHeader
                        header_type=&playlist.r#type
                        name=&playlist.name
                        authors=converted_owner
                        image=&playlist.images[0].url
                        total=playlist.tracks.total
                        total_duration=total_duration
                        release_date=release_date
                         />
                    <TrackTable tracks=&tracks headers=headers />
                </>
            }
        } else if !self.state.get_playlist_loaded {
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