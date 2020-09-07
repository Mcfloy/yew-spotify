use crate::types::Artist;
use yew::prelude::*;
use crate::components::{TopTracks, ArtistAlbums};

pub struct AppBanner {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub artist: Artist,
}

pub enum Msg {}

impl Component for AppBanner {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let image = match &self.props.artist.images {
            Some(images) => {
                if images.is_empty() {
                    html! {}
                } else {
                    html! { <img src={images[0].url.clone()} /> }
                }
            },
            None => html! {}
        };

        let followers = match &self.props.artist.followers {
            Some(followers) => format!("{} followers", followers.total),
            None => String::from("")
        };

        let genres = match &self.props.artist.genres {
            Some(genres) => {
                let genres_node: Vec<Html> = genres.iter().map(|genre| {
                    html! {
                        <li>{genre}</li>
                    }
                }).collect();
                html! {
                    <ul class="genres">
                        { genres_node }
                    </ul>
                }
            },
            None => html! {}
        };
        html! {
            <header class="banner">
                <div class="image-container">
                    { image }
                </div>
                <div class="side-container">
                    <h1>{ &self.props.artist.name }</h1>
                    <span class="informations">
                        {followers}
                    </span>
                    { genres }
                    <h4>{"Top tracks"}</h4>
                    <TopTracks artist_id=self.props.artist.id.clone() />
                    <h4>{"Albums"}</h4>
                    <ArtistAlbums artist_id=self.props.artist.id.clone() album_group={"album"} />
                    <h4>{"Singles and EP"}</h4>
                    <ArtistAlbums artist_id=self.props.artist.id.clone() album_group={"single"} />
                    <h4>{"Playlists of "} {self.props.artist.name.clone()}</h4>
                    <ArtistAlbums artist_id=self.props.artist.id.clone() album_group={"compilation"} />
                    <h4>{"Appears on"}</h4>
                    <ArtistAlbums artist_id=self.props.artist.id.clone() album_group={"appears_on"} />
                </div>
            </header>
        }
    }
}