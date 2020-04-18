use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Profile {
    pub uid: String,
    pub follower_count: u32,
}
