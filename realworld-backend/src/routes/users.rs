use rocket::{
    serde::json::{json, Json, Value},
    Route,
};

use crate::models::{
    UserLoginWrapper, UserRegisterWrapper, UserResp, UserUpdateWrapper, UserWrapper,
};

#[post("/users", data = "<user>")]
pub async fn register(user: Json<UserRegisterWrapper>) -> Json<UserWrapper> {
    let user = user.into_inner().user;
    Json(UserWrapper {
        user: UserResp {
            email: user.email,
            token: "jwt.token.here".to_string(),
            username: user.username,
            bio: None,
            image: None,
        },
    })
}

#[post("/users/login", data = "<user>")]
pub async fn login(user: Json<UserLoginWrapper>) -> Json<UserWrapper> {
    let user = user.into_inner().user;
    Json(UserWrapper {
        user: UserResp {
            email: user.email,
            token: "jwt.token.here".to_string(),
            username: "dunmy_username".to_string(),
            bio: Some("I work at statefram".to_string()),
            image: None,
        },
    })
}

#[get("/user")]
pub async fn current_user() -> Json<UserWrapper> {
    Json(UserWrapper {
        user: UserResp {
            email: "717513736@qq.com".to_string(),
            token: "jwt.token.here".to_string(),
            username: "Zhong".to_string(),
            bio: Some("I work at statefram".to_string()),
            image: None,
        },
    })
}

#[put("/user", data = "<user>")]
pub async fn update_user(user: Json<UserUpdateWrapper>) -> Json<UserWrapper> {
    let user = user.into_inner().user;
    Json(UserWrapper {
        user: UserResp {
            email: user.email.unwrap_or("717513736@qq.com".to_string()),
            token: "jwt.token.here".to_string(),
            username: user.username.unwrap_or("Zhong".to_string()),
            bio: user.bio,
            image: user.image,
        },
    })
}

pub fn get_routes() -> Vec<Route> {
    routes![register, login, current_user, update_user]
}
