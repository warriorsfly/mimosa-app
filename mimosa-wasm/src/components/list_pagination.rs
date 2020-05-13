use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};

const PAGE_SIZE: u32 = 10;

pub struct ListPagination {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub articles_count: u32,
    pub current_page: u32,
    pub callback: Callback<u32>,
}

pub enum Msg {
    PaginationChanged(u32),
}

impl Component for ListPagination {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PaginationChanged(index) => {
                self.props.callback.emit(index);
            }
        }

        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        true
    }
    fn view(&self) -> Html {
        if self.props.articles_count < PAGE_SIZE {
            return html! {};
        }

        let max_page = (self.props.articles_count as f32 / 10.0).ceil() as u32;
        let mut pages: Vec<u32> = vec![];
        for page in 0..max_page {
            pages.push(page);
        }

        html! {
            <nav>
                <ul class="pagination">
                {for pages.iter().map(|page| {
                    let is_current = page == &self.props.current_page;
                    let page_item_class = if is_current {
                        "page-item active"
                    } else {
                        "page-item"
                    };
                    let page = page.clone();
                    let onclick = self.link.callback(move |ev: MouseEvent| {ev.prevent_default(); Msg::PaginationChanged(page)});
                    html! {
                        <li
                            class=page_item_class
                            onclick=onclick>
                            <a class="page-link" href="">{page + 1}</a>

                        </li>
                    }
                })}
                </ul>
            </nav>
        }
    }
}
