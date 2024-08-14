use crate::models::user::{Profile, ProfileResponse};
use rocket::{delete, get, post, routes, serde::json::Json};

// 获取用户资料
#[get("/profiles/<username>")]
async fn get_profile(username: String) -> Json<ProfileResponse> {
    Json(ProfileResponse {
        profile: Profile {
            username,
            bio: Some("I work at statefarm".to_string()),
            image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
            following: false,
        },
    })
}

// 关注用户
#[post("/profiles/<username>/follow")]
async fn follow_user(username: String) -> Json<ProfileResponse> {
    Json(ProfileResponse {
        profile: Profile {
            username,
            bio: Some("I work at statefarm".to_string()),
            image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
            following: true,
        },
    })
}

// 取消关注用户
#[delete("/profiles/<username>/follow")]
async fn unfollow_user(username: String) -> Json<ProfileResponse> {
    Json(ProfileResponse {
        profile: Profile {
            username,
            bio: Some("I work at statefarm".to_string()),
            image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
            following: false,
        },
    })
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_profile, follow_user, unfollow_user]
}
