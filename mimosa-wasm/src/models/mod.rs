mod article;
mod auth;
mod comment;
mod profile;
mod tag;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use article::{
    ArticleInfo, ArticleInfoDto, ArticleInfoDtoWrapper, ArticleInfoWrapper, ArticleListInfo,
};
pub use auth::{LoginInfo, LoginInfoWrapper, UserInfo, UserInfoWrapper};
pub use comment::{Comment, CommentDto, CommentDtoWrapper, CommentList, CommentWrapper};
pub use profile::Profile;
pub use tag::TagListInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
