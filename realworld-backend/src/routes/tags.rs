use crate::models::tags::TagsResponse;
use rocket::serde::json::Json;

#[get("/tags")]
async fn get_tags() -> Json<TagsResponse> {
    let tags = vec![
        "rust".to_string(),
        "rocket".to_string(),
        "realworld".to_string(),
        "api".to_string(),
    ];

    Json(TagsResponse { tags })
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_tags]
}
