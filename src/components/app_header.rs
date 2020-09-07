use yew::prelude::*;
use crate::utils::parse_time_to_long_string;
use yew_router::prelude::RouterAnchor;
use crate::route::Route;
use crate::types::Artist;

pub struct AppHeader {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String,
    pub authors: Vec<Artist>, // Authors also contains the
    pub image: String,
    pub total: u32,
    pub total_duration: u32,
    pub release_date: Option<String>,
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
        type Anchor = RouterAnchor<Route>;

        let release_date_node = if let Some(release_date) = &self.props.release_date {
            html! {
                <>{&release_date}{" • "}</>
            }
        } else {
            html! {}
        };

        let authors_node: Vec<Html> = self.props.authors.iter()
            .map(|author| {
                if author.r#type == "artist" {
                    html! {
                        <Anchor route=Route::ArtistDetail(author.id.clone()) classes="link_anchor">
                            <b>{&author.name}</b>
                        </Anchor>
                    }
                } else {
                    html! {
                        <b>{&author.name}</b>
                    }
                }
            })
            .collect();

        html! {
            <header class="header">
                <div class="image-container">
                    <img src={&self.props.image} />
                </div>
                <div class="side-container">
                    <h1>{&self.props.name}</h1>
                    <h5>
                        <span class="type">{&self.props.header_type}</span> {"Par "}
                        { authors_node }
                    </h5>
                    <span class="informations">{release_date_node}{&self.props.total}{" titres, "}{parse_time_to_long_string(&self.props.total_duration)}</span>
                </div>
            </header>
        }
    }
}