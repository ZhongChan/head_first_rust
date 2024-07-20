use crate::{
    apilayer::{self},
    store::Store,
    types::{
        account::Session,
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

#[instrument]
pub async fn get_questions(
    session: Session,
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "ch_09",Level::INFO,"querying questions");

    let account_id = session.account_id;
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?; // use ? get Pagination strut
    }

    info!(pagination = false);
    let res = match store
        .get_questions(pagination.limit, pagination.offset, account_id)
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
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
    match store.add_question(question, account_id).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn update_question_tokio_spawn(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let content = tokio::spawn(apilayer::check_profanity(question.content));
        let title = tokio::spawn(apilayer::check_profanity(question.title));

        let (title, content) = (title.await.unwrap(), content.await.unwrap());

        if title.is_err() {
            return Err(warp::reject::custom(title.unwrap_err()));
        }

        if content.is_err() {
            return Err(warp::reject::custom(title.unwrap_err()));
        }

        let question = Question {
            id: question.id,
            title: title.unwrap(),
            content: content.unwrap(),
            tags: question.tags,
        };

        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(err) => Err(warp::reject::custom(err)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    match store.delete_question(id, account_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
