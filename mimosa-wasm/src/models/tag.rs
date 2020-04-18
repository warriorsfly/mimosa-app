use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagListInfo {
    pub tags: Vec<String>,
}
