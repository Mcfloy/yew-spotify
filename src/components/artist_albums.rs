use yew::prelude::*;
use crate::types::{SimplifiedAlbum, Paging};
use yew::services::fetch::{FetchTask};
use anyhow::Error;
use crate::api::FetchResponse;
use yew::format::Json;
use crate::api;
use yew_router::prelude::RouterAnchor;
use crate::route::Route;

pub(crate) struct ArtistAlbums {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

#[derive(Properties, Clone)]
pub struct Props {
    pub artist_id: String,
    pub album_group: String,
}

struct State {
    artist_albums: Option<Vec<SimplifiedAlbum>>,
    get_artist_albums_error: Option<Error>,
    get_artist_albums_loaded: bool
}

pub enum Msg {
    GetArtistAlbums,
    GetArtistAlbumsSuccess(Vec<SimplifiedAlbum>),
    GetArtistAlbumsError(Error)
}

impl Component for ArtistAlbums {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetArtistAlbums);

        Self {
            props,
            state: State {
                artist_albums: None,
                get_artist_albums_error: None,
                get_artist_albums_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetArtistAlbums => {
                self.state.get_artist_albums_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<Paging<SimplifiedAlbum>>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(artist_albums_paging) => Msg::GetArtistAlbumsSuccess(artist_albums_paging.items),
                            Err(err) => Msg::GetArtistAlbumsError(err)
                        }
                    });

                self.task = Some(api::get_artist_albums(self.props.artist_id.clone(), vec![self.props.album_group.clone()], None, None, handler));
                true
            },
            Msg::GetArtistAlbumsSuccess(artist_albums) => {
                self.state.get_artist_albums_loaded = true;
                self.state.artist_albums = Some(artist_albums);
                true
            },
            Msg::GetArtistAlbumsError(error) => {
                self.state.get_artist_albums_loaded = true;
                self.state.get_artist_albums_error = Some(error);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        self.link.send_message(Msg::GetArtistAlbums);
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        if let Some(ref artist_albums) = &self.state.artist_albums {
            let artist_albums_node: Vec<Html> = artist_albums.iter()
                .map(|artist_album| {
                    html! {
                        <li>
                            <img src={&artist_album.images[0].url} />
                            <Anchor route=Route::AlbumDetail(artist_album.id.clone()) classes="link_anchor">
                                {&artist_album.name}
                            </Anchor>
                            <span>{&artist_album.release_date}</span>
                        </li>
                    }
                })
                .collect();
            html! {
                <ul class="album-list">
                    { artist_albums_node }
                </ul>
            }
        } else if !self.state.get_artist_albums_loaded {
            html! {
                <div>{"Loading ..."}</div>
            }
        } else if let Some(error) = &self.state.get_artist_albums_error {
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