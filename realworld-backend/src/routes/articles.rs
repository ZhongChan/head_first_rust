use crate::models::article::{
    ArticleResponse, AuthorResponse, MultipleArticlesWrapper, NewArticleWrapper,
    SingleArticleWrapper, UpdateArticleWrapper,
};
use chrono::Utc;
use rocket::{delete, get, post, put, routes, serde::json::Json};

// 获取所有文章
#[get("/articles")]
async fn get_articles() -> Json<MultipleArticlesWrapper> {
    Json(MultipleArticlesWrapper {
        articles: vec![ArticleResponse {
            slug: "how-to-train-your-dragon".to_string(),
            title: "How to train your dragon".to_string(),
            description: "Ever wonder how?".to_string(),
            body: "It takes a Jacobian".to_string(),
            tag_list: vec!["dragons".to_string(), "training".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: false,
            favorites_count: 0,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        }],
        articles_count: 1,
    })
}

// 获取单篇文章
#[get("/articles/<slug>")]
async fn get_article(slug: String) -> Json<SingleArticleWrapper> {
    Json(SingleArticleWrapper {
        article: ArticleResponse {
            slug,
            title: "How to train your dragon".to_string(),
            description: "Ever wonder how?".to_string(),
            body: "It takes a Jacobian".to_string(),
            tag_list: vec!["dragons".to_string(), "training".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: false,
            favorites_count: 0,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        },
    })
}

// 创建文章
#[post("/articles", data = "<article>")]
async fn create_article(article: Json<NewArticleWrapper>) -> Json<SingleArticleWrapper> {
    let article = article.into_inner().article;
    Json(SingleArticleWrapper {
        article: ArticleResponse {
            slug: "how-to-train-your-dragon".to_string(),
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: false,
            favorites_count: 0,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        },
    })
}

// 更新文章
#[put("/articles/<slug>", data = "<article>")]
async fn update_article(
    slug: String,
    article: Json<UpdateArticleWrapper>,
) -> Json<SingleArticleWrapper> {
    let article = article.into_inner().article;
    Json(SingleArticleWrapper {
        article: ArticleResponse {
            slug,
            title: article
                .title
                .unwrap_or("Did you train your dragon?".to_string()),
            description: article.description.unwrap_or("How did it go?".to_string()),
            body: article.body.unwrap_or("It was a success".to_string()),
            tag_list: article
                .tag_list
                .unwrap_or(vec!["dragons".to_string(), "success".to_string()]),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: false,
            favorites_count: 0,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        },
    })
}

// 删除文章
#[delete("/articles/<slug>")]
async fn delete_article(slug: String) -> rocket::http::Status {
    println!("{}", slug);
    rocket::http::Status::NoContent
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_articles,
        get_article,
        create_article,
        update_article,
        delete_article
    ]
}
