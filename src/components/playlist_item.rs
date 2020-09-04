use yew::prelude::*;
use crate::route::Route;
use yew_router::components::RouterAnchor;

pub struct PlaylistItem {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: String,
    pub name: String,
}

impl Component for PlaylistItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        html! {
            <Anchor route=Route::PlaylistDetail(self.props.id.to_string())>
                <li>{&self.props.name}</li>
            </Anchor>
        }
    }
}