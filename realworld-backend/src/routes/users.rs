use rocket::http::Status;
use rocket::response::status;
use rocket::{serde::json::Json, Route};

use crate::database::user::NewUser;
use crate::models::user::{
    UserLoginWrapper, UserRegisterWrapper, UserResp, UserUpdateWrapper, UserWrapper,
};

use crate::database::Db;
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[post("/users", data = "<user>")]
pub async fn register(
    user: Json<RegisterUser>,
    conn: Db,
) -> Result<status::Created<Json<UserResponse>>, Status> {
    let password_hash = hash(&user.password, 4).map_err(|_| Status::InternalServerError)?;
    let new_user = NewUser {
        email: user.email.clone(),
        username: user.username.clone(),
        password_hash,
    };

    conn.run(move |c| diesel::insert_into(users).values(&new_user).execute(c))
        .await
        .map_err(|_| Status::InternalServerError)?;

    let user_response = UserResponse {
        email: user.email.clone(),
        token: "fake-jwt-token".to_string(), // 这里应该生成一个实际的 JWT token
        username: user.username.clone(),
        bio: None,
        image: None,
    };

    Ok(status::Created::new("/").body(Json(user_response)))
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
