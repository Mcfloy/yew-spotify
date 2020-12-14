use yew::prelude::*;

pub struct PlayerInformation {}

impl Component for PlayerInformation {
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
            <div class="relative_container">
                <div id="player_infos">
                    <img id="player_cover" />
                    <span id="player_song_name"></span>
                    <span id="player_artist"></span>
                </div>
                <div id="player_commands">
                    <button id="previous_button">
                        <span class="material-icons">
                            {"skip_previous"}
                        </span>
                    </button>
                    <button id="play_button">
                        <span class="material-icons">
                            {"play_arrow"}
                        </span>
                    </button>
                    <button id="next_button">
                        <span class="material-icons">
                            {"skip_next"}
                        </span>
                    </button>
                </div>
                <div id="player_progression">
                    <span id="player_progression_position"></span>
                    <input type="range" id="player_slider" min="0" max="100" step="any" value="0" />
                    <div id="player_progression_lower_track"></div>
                    <span id="player_progression_duration"></span>
                </div>
                <input type="range" id="player_volume" min="0" max="100" step="1" value="50" />
            </div>
        }
    }
}