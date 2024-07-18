use crate::{
    apilayer::{self},
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "ch_07",Level::INFO,"querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?; // use ? get Pagination strut
    }

    info!(pagination = false);
    let res = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let content = match apilayer::check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let title = match apilayer::check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    // add question
    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    let content = match apilayer::check_profanity(question.content).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let title = match apilayer::check_profanity(question.title).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let question = Question {
        id: question.id,
        title,
        content,
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
