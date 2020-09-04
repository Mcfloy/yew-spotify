use yew::prelude::*;
use crate::types::{Track};
use crate::utils::{play, parse_time_to_short_string};
use yew_router::prelude::RouterAnchor;
use crate::route::Route;

pub struct TrackTable {
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub tracks: Vec<Track>,
    pub headers: Vec<TrackTableHeader>
}

#[derive(PartialEq, Clone)]
pub enum TrackTableHeader {
    TrackNumber,
    Name,
    Artist,
    Album,
    Duration,
    AddedAt,
    Popularity
}

pub enum Msg {
    PlayTrack(String)
}

impl Component for TrackTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayTrack(spotify_uri) => {
                play(spotify_uri);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let tracks: Vec<Html> = self.props.tracks.iter()
            .map(|track| self.view_track(track, self.props.headers.clone()))
            .collect();

        html! {
            <table class="track-table">
                <tbody>
                    { tracks }
                </tbody>
            </table>
        }
    }
}

impl TrackTable {
    fn view_track(&self, track: &Track, headers: Vec<TrackTableHeader>) -> Html {
        type Anchor = RouterAnchor<Route>;

        let track = &track;
        let onclick_play = {
            let uri = track.uri.clone();
            self.link.callback(move |_| Msg::PlayTrack(uri.clone()))
        };
        let artists: Vec<Html> = track.artists
            .iter()
            .map(|artist| {
                html! {
                    <Anchor route=Route::ArtistDetail(artist.id.clone()) classes="link_anchor">
                        <span>{artist.name.clone()}</span>
                    </Anchor>
                }
            })
            .collect();

        let track_number_node = if headers.contains(&TrackTableHeader::TrackNumber) {
            html! {
                <span class="track-number">{&track.track_number}</span>
            }
        } else {
            html! {}
        };

        let play_node = html! {
            <td class="play">
                { track_number_node }
                <button onclick=onclick_play>
                    <span class="material-icons">
                        {"play_arrow"}
                    </span>
                </button>
            </td>
        };

        let name_node = if headers.contains(&TrackTableHeader::Name) {
            html! {
                <td class="name">{&track.name}</td>
            }
        } else {
            html! {}
        };

        let artist_node = if headers.contains(&TrackTableHeader::Artist) {
            html! {
                <td class="artist">
                    { artists }
                </td>
            }
        } else {
            html! {}
        };

        let album_node = if headers.contains(&TrackTableHeader::Album) {
            if let Some(album) = &track.album {
                html! {
                    <td class="album">
                        <Anchor route=Route::AlbumDetail(album.id.clone()) classes="link_anchor">
                            <span>{&album.name}</span>
                        </Anchor>
                    </td>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };

        let added_at_node = if headers.contains(&TrackTableHeader::AddedAt) {
            if let Some(added_at) = &track.added_at {
                html! {
                    <td class="added_at">{added_at.format("%F")}</td>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };

        let duration_node = if headers.contains(&TrackTableHeader::Duration) {
            html! {
                <td class="duration">{parse_time_to_short_string(&track.duration_ms)}</td>
            }
        } else {
            html! {}
        };

        let popularity_node = if headers.contains(&TrackTableHeader::Popularity) {
            if let Some(popularity) = &track.popularity {
                html! {
                    <td class="popularity">{popularity}</td>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };

        let options_node = html! { <td></td> };

        html! {
            <tr>
                { play_node }
                <td class="love">
                    <span class="material-icons">
                        {"favorite_border"}
                    </span>
                </td>
                { name_node }
                { artist_node }
                { album_node }
                { added_at_node }
                { options_node }
                { duration_node }
                { popularity_node }
            </tr>
        }
    }
}