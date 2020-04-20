use serde::{Deserialize, Serialize};

use super::Profile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleInfo {
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
pub struct ArticleInfoWrapper {
    pub article: ArticleInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleListInfo {
    pub articles: Vec<ArticleInfo>,
    pub articles_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleInfoDto {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleInfoDtoWrapper {
    pub article: ArticleInfoDto,
}
