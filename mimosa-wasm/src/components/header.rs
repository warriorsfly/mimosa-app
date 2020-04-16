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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {}
}
