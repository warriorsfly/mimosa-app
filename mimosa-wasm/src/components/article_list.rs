use crate::{errors::Error, models::ArticleListInfo, services::ArticleService};

use super::article_preview::ArticlePreview;
use super::list_pagination::ListPagination;
use yew::{
    html, services::fetch::FetchTask, Callback, Component, ComponentLink, Html, Properties,
    ShouldRender,
};

/// list of articles component
pub struct ArticleList {
    articles: ArticleService,
    article_list: Option<ArticleListInfo>,
    response: Callback<Result<ArticleListInfo, Error>>,
    task: Option<FetchTask>,
    current_page: u32,
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ArticleList {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            articles: ArticleService::new(),
            article_list: None,
            response: link.callback(Msg::Response),
            task: None,
            current_page: 0,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(article_list)) => {
                self.article_list = Some(article_list);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
            Msg::PaginationChanged(current_page) => {
                self.current_page = current_page;
                self.request();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.props = _props;
        self.current_page = 0;
        self.request();
        false
    }
    fn view(&self) -> Html {
        if let Some(article_list) = &self.article_list {
            if !article_list.articles.is_empty() {
                let callback = self.link.callback(Msg::PaginationChanged);
                html! {
                    <>
                        {for article_list.articles.iter().map(|article| {
                            html! { <ArticlePreview article=article /> }
                        })}
                        <ListPagination
                            articles_count=article_list.articles_count
                            current_page=self.current_page
                            callback=callback />
                    </>
                }
            } else {
                html! {
                    <div class="article-preview">{ "No articles are here... yet." }</div>
                }
            }
        } else {
            html! {
                <div class="article-preview">{ "Loading..." }</div>
            }
        }
    }
}

impl ArticleList {
    fn request(&mut self) {
        match self.props.filter.clone() {
            ArticleListFilter::All => {
                self.task = Some(
                    self.articles
                        .get_all(self.current_page, self.response.clone()),
                );
            }
            ArticleListFilter::ByAuthor(author) => {
                self.task = Some(self.articles.get_by_author(
                    author,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::ByTag(tag) => {
                self.task = Some(self.articles.get_by_tag(
                    tag,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::Favorited(author) => {
                self.task = Some(self.articles.favorited_by(
                    author,
                    self.current_page,
                    self.response.clone(),
                ));
            }
            ArticleListFilter::Feed => {
                self.task = Some(self.articles.feed(self.response.clone()));
            }
        }
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub filter: ArticleListFilter,
}

pub enum Msg {
    Response(Result<ArticleListInfo, Error>),
    PaginationChanged(u32),
}

#[derive(Debug, Clone)]
pub enum ArticleListFilter {
    All,
    ByAuthor(String),
    ByTag(String),
    Favorited(String),
    Feed,
}
