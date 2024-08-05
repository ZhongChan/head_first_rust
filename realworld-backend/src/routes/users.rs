use rocket::serde::json::{json, Value};

#[get("/user")]
pub async fn get_user() -> Option<Value> {
    Some(json!({"message":"Hello,user!"}))
}
