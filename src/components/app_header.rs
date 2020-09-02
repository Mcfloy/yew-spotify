use yew::prelude::*;
use crate::utils::parse_time_to_string;

pub struct AppHeader {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub author: String,
    pub image: String,
    pub total: u32,
    pub total_duration: u32,
    pub release_date: String,
    pub header_type: String
}

pub enum Msg {}

impl Component for AppHeader {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
                <div class="image-container">
                    <img src={&self.props.image} />
                </div>
                <div class="side-container">
                    <h1>{&self.props.name}</h1>
                    <h5><span class="type">{&self.props.header_type}</span> {"Par "}<b>{&self.props.author}</b></h5>
                    <span class="informations">{&self.props.release_date}{" • "}{&self.props.total}{" titres, "}{parse_time_to_string(&self.props.total_duration)}</span>
                </div>
            </header>
        }
    }
}