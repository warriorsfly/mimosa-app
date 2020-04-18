use serde::{Deserialize, Serialize};

use super::Profile;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub body: String,
    pub author: Profile,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentWrapper {
    pub comment: Comment,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommentDto {
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentDtoWrapper {
    pub comment: CommentDto,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentList {
    pub comments: Vec<Comment>,
}
