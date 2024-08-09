use rocket::serde::json::{json, Value};

#[post("/users")]
pub async fn register() {}

#[post("/users/login")]
pub async fn login() {}

#[get("/user")]
pub async fn current_user() -> Option<Value> {
    Some(json!({"message":"Hello,user!"}))
}

#[put("/user")]
pub async fn update_user() {}
