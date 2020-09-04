use crate::types::Artist;
use yew::prelude::*;

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
        html! {
            <header>
                { &self.props.artist.name }
            </header>
        }
    }
}