use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;

use crate::pages::{Home, PlaylistDetail, AlbumDetail, ArtistDetail};
use crate::route::Route;
use crate::utils;
use crate::components::PlaylistList;
use crate::components::PlayerInformation;
use crate::components::CurrentUserInformation;
use crate::components::SearchBar;
use crate::types::PrivateUser;
use anyhow::Error;
use crate::api::FetchResponse;
use yew::services::fetch::FetchTask;
use crate::api;
use yew::format::Json;


pub struct App {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

struct State {
    current_user_error: Option<Error>,
    current_user_loaded: bool
}

pub enum Msg {
    GetCurrentUserProfile,
    GetCurrentUserProfileSuccess(PrivateUser),
    GetCurrentUserProfileError(Error)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetCurrentUserProfile);
        Self {
            state: State {
                current_user_error: None,
                current_user_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCurrentUserProfile => {
                self.state.current_user_loaded = false;
                let handler = self
                    .link
                    .callback(move |response: FetchResponse<PrivateUser>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(user) => Msg::GetCurrentUserProfileSuccess(user),
                            Err(err) => Msg::GetCurrentUserProfileError(err),
                        }
                    });

                self.task = Some(api::get_current_user_profile(handler));
                true
            },
            Msg::GetCurrentUserProfileSuccess(user) => {
                self.state.current_user_loaded = true;
                utils::setItem(String::from("user"), serde_json::to_string(&user).unwrap());
                true
            },
            Msg::GetCurrentUserProfileError(error) => {
                self.state.current_user_loaded = true;
                self.state.current_user_error = Some(error);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if !self.state.current_user_loaded {
            return html! {
                <div>{"loading..."}</div>
            }
        } else if let Some(error) = &self.state.current_user_error {
            return html! {
                <div>
                    <span>{"Error:"} {error}</span>
                </div>
            }
        }
        type Anchor = RouterAnchor<Route>;

        let render = Router::render(|switch: Route| match switch {
            Route::PlaylistDetail(id) => html! { <PlaylistDetail id=id /> },
            Route::AlbumDetail(id) => html! { <AlbumDetail id=id /> },
            Route::ArtistDetail(id) => html! { <ArtistDetail id=id /> },
            Route::HomePage => html! { <Home /> },
        });

        html! {
            <main>
                <div id="banner_top">
                    <SearchBar />
                    <CurrentUserInformation />
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