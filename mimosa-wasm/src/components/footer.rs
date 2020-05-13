use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

use crate::routes::AppRoute;
pub struct Footer {}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer>
                <div class="container">
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="logo-font">{"mimosa"}</RouterAnchor<AppRoute>>

                    <span class="attribution">
                        { "© 2020. An interactive learning project from" }
                        <a href="https://github.com/warriorsfly"> { "Warriorsfly" } </a>
                        { ". Code licensed under MIT." }
                    </span>
                </div>
            </footer>
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
}
