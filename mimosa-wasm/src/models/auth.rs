use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    pub user_role: i32,
    pub register_source: i32,
    pub nick_name: String,
    pub gender: i32,
    pub signature: String,
    pub email: String,
    pub avatar: String,
    pub avatar200: String,
    pub avatar_source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

// #[derive(Serialize, Deserialize, Clone, Debug, Default)]
// pub struct UserUpdateDto {
//     pub user_role: i32,
//     pub register_source: i32,
//     pub nick_name: String,
//     pub gender: i32,
//     pub signature: String,
//     pub email: String,
//     pub avatar: String,
//     pub avatar200: String,
//     pub avatar_source: String,
// }

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LoginInfo {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}
