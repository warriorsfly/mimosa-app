pub mod auth;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use auth::UserInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
