use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub img: String,
    pub following: bool,
}
