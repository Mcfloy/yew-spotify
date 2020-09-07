use yew::prelude::*;
use crate::types::PrivateUser;
use crate::utils;

pub struct CurrentUserInformation {
    state: State
}

pub struct State {
    user: PrivateUser
}

impl Component for CurrentUserInformation {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let user_str = utils::getItem(String::from("user"));
        let user = serde_json::from_str::<PrivateUser>(user_str.as_str()).unwrap();
        Self {
            state: State {
                user
            }
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let display_name = if let Some(name) = &self.state.user.display_name {
            name.clone()
        } else {
          String::from("")
        };
        html! {
            <div id="current_user_information">
                <span>{display_name}</span>
            </div>
        }
    }
}