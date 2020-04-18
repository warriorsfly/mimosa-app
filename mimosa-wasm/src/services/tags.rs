use super::Requests;
use crate::{errors::Error, models::TagListInfo};
use yew::{services::fetch::FetchTask, Callback};

#[derive(Default, Debug)]
pub struct Tags {
    requests: Requests,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn default() -> Self {
        Self {
            requests: Requests::default(),
        }
    }

    pub fn get_all(&mut self, callback: Callback<Result<TagListInfo, Error>>) -> FetchTask {
        self.requests
            .get::<TagListInfo>("/tags".to_string(), callback)
    }
}
