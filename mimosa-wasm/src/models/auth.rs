use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub user_role: i32,
    pub register_source: i32,
    pub nick_name: String,
    pub gender: i32,
    pub birthday: Option<chrono::NaiveDateTime>,
    pub signature: String,
    pub mobile: String,
    pub mobile_bind_time: Option<chrono::NaiveDateTime>,
    pub email: String,
    pub email_bind_time: Option<chrono::NaiveDateTime>,
    pub avatar: String,
    pub avatar200: String,
    pub avatar_source: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub push_token: String,
}
