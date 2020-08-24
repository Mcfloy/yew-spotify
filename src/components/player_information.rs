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
            <>
                <div id="player_infos">
                    <img id="player_cover" />
                    <span id="player_song_name"></span>
                    <span id="player_artist"></span>
                </div>
                <span id="player_progression"></span>
            </>
        }
    }
}