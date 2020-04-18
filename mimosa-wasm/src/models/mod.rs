mod auth;
mod tags;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use auth::UserInfo;
pub use tags::TagListInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
