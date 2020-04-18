mod article;
mod auth;
mod comment;
mod profile;
mod tag;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use article::{Article, ArticleDto, ArticleDtoWrapper, ArticleList, ArticleWrapper};
pub use auth::UserInfo;
pub use comment::{Comment, CommentDto, CommentDtoWrapper, CommentList, CommentWrapper};
pub use profile::Profile;
pub use tag::TagListInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
