use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;

use crate::pages::{Home, PlaylistDetail};
use crate::route::Route;

use crate::components::PlaylistList;
use crate::components::PlayerInformation;
use crate::components::CurrentUserInformation;
use crate::components::SearchBar;


pub struct App {}

impl Component for App {
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
        type Anchor = RouterAnchor<Route>;

        let render = Router::render(|switch: Route| match switch {
            Route::PlaylistDetail(id) => html! { <PlaylistDetail id=id/> },
            Route::HomePage => html! { <Home /> },
        });

        html! {
            <main>
                <div id="banner_top">
                    <SearchBar />
                    <CurrentUserInformation name={"Lucas Perreau"} />
                </div>
                <div id="banner_left">
                    <Anchor route=Route::HomePage>{"Accueil"}</Anchor>
                    <div class="overflow-handler">
                        <div>
                            <h3>{"Bibliothèque"}</h3>
                        </div>
                        <PlaylistList />
                    </div>
                </div>
                <div id="container">
                    <div class="overflow-handler">
                        <Router<Route, ()> render=render/>
                    </div>
                </div>
                <div id="banner_bottom">
                    <PlayerInformation />
                </div>
            </main>

        }
    }
}