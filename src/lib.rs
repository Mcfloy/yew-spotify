use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct SpotifyApp {
}

impl Component for SpotifyApp {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <div id="banner_top"></div>
                <div id="banner_left">
                    <div id="my_playlists"></div>
                </div>
                <div id="container"></div>
                <div id="banner_bottom">
                    <div id="player_infos">
                        <img id="player_cover" />
                        <span id="player_song_name"></span>
                        <span id="player_artist"></span>
                    </div>
                    <span id="player_progression"></span>
                </div>
            </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<SpotifyApp>::new().mount_to_body();
}