use serde::{Deserialize, Serialize};

use super::Profile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub discription: String,
    pub body: String,
    pub author: Profile,
    pub tag_list: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub favorited: bool,
    pub favorites_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleWrapper {
    pub article: Article,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleList {
    pub articles: Vec<Article>,
    pub articles_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleDto {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleDtoWrapper {
    pub article: ArticleDto,
}
