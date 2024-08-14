use serde::Serialize;

#[derive(Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
