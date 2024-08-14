use crate::models::article::{
    ArticleResponse, AuthorResponse, CommentResponse, MultipleArticlesWrapper,
    MultipleCommentsWrapper, NewArticleWrapper, NewCommentWrapper, SingleArticleWrapper,
    SingleCommentWrapper, UpdateArticleWrapper,
};
use chrono::Utc;
use rocket::{delete, get, post, put, routes, serde::json::Json};

// 获取关注用户的文章列表
#[get("/articles/feed")]
async fn get_articles_feed() -> Json<MultipleArticlesWrapper> {
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

// 添加评论到文章
#[post("/articles/<slug>/comments", data = "<comment>")]
async fn add_comment(slug: String, comment: Json<NewCommentWrapper>) -> Json<SingleCommentWrapper> {
    let comment = comment.into_inner().comment;
    Json(SingleCommentWrapper {
        comment: CommentResponse {
            id: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            body: comment.body,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        },
    })
}

// 获取文章的评论
#[get("/articles/<slug>/comments")]
async fn get_comments(slug: String) -> Json<MultipleCommentsWrapper> {
    Json(MultipleCommentsWrapper {
        comments: vec![CommentResponse {
            id: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            body: "Great article!".to_string(),
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        }],
    })
}

// 删除文章的评论
#[delete("/articles/<slug>/comments/<id>")]
async fn delete_comment(slug: String, id: i32) -> rocket::http::Status {
    println!("Article: {}, Comment ID: {}", slug, id);
    rocket::http::Status::NoContent
}

// 添加文章到收藏
#[post("/articles/<slug>/favorite")]
async fn favorite_article(slug: String) -> Json<SingleArticleWrapper> {
    Json(SingleArticleWrapper {
        article: ArticleResponse {
            slug,
            title: "How to train your dragon".to_string(),
            description: "Ever wonder how?".to_string(),
            body: "It takes a Jacobian".to_string(),
            tag_list: vec!["dragons".to_string(), "training".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            favorited: true,
            favorites_count: 1,
            author: AuthorResponse {
                username: "jake".to_string(),
                bio: Some("I work at statefarm".to_string()),
                image: Some("https://i.stack.imgur.com/xHWG8.jpg".to_string()),
                following: false,
            },
        },
    })
}

// 从收藏中移除文章
#[delete("/articles/<slug>/favorite")]
async fn unfavorite_article(slug: String) -> Json<SingleArticleWrapper> {
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
        delete_article,
        get_articles_feed,
        add_comment,
        get_comments,
        delete_comment,
        favorite_article,
        unfavorite_article
    ]
}
