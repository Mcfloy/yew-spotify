use yew::prelude::*;

pub struct CurrentUserInformation {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub name: String
}

impl Component for CurrentUserInformation {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="current_user_information">
                <span>{&self.props.name}</span>
            </div>
        }
    }
}