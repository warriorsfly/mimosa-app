use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::models::UserInfo;
use crate::routes::AppRoute;
pub struct Header {
    pub props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub current: Option<UserInfo>,
}

pub enum Msg {}

impl Compenent for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home class="navbar-brand">
                        {"Mimosa"}
                    </RouterAnchor<AppRoute>>
                    {
                        if let Some(account) = &self.props.current{
                            self.logged_in_view(&account)
                        }else{
                            self.logged_out_view()
                        }
                    }
                </div>
            </nav>
        }
    }
}

impl Header {
    fn logged_out_view(&self) -> Html {
        html! {
            <ul class="nav navbar-nav pull-xs-right">
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav-link">
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Login classes="nav-link">
                        { "Sign in" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li class="nav-item">
                    <RouterAnchor<AppRoute> route=AppRoute::Register classes="nav-link">
                        { "Sign up" }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }
}
