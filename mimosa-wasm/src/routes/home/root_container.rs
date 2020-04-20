use yew::{Component, Properties};

pub struct RootContainer {
    props: Props,
    tab: Tab,
    // filter: A,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub tag: Option<String>,
}

#[derive(Clone)]
pub enum Msg {
    TabChanged(Tab),
    Ignore,
}

#[derive(PartialEq, Clone)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

impl Component for RootContainer {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        todo!()
    }
    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        todo!()
    }
    fn view(&self) -> yew::Html {
        todo!()
    }
}
