use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResp {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize)]
pub struct UserWrapper {
    pub user: UserResp,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLoginWrapper {
    pub user: UserLogin,
}

#[derive(Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRegisterWrapper {
    pub user: UserRegister,
}

#[derive(Deserialize)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct UserUpdateWrapper {
    pub user: UserUpdate,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}
