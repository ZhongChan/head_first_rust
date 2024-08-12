use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

fn serialize_iso8601<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}

#[derive(Serialize)]
pub struct AuthorResponse {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponse {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    #[serde(serialize_with = "serialize_iso8601")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_iso8601")]
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: u32,
    pub author: AuthorResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleArticleWrapper {
    pub article: ArticleResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesWrapper {
    pub articles: Vec<ArticleResponse>,
    pub articles_count: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleWrapper {
    pub article: NewArticle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleWrapper {
    pub article: UpdateArticle,
}
