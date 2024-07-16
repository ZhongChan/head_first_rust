use crate::{
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};
use handle_errors::Error;
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
        Err(err) => return Err(warp::reject::custom(Error::DatabaseQueryError(err))),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(err) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(err)));
    }
    Ok(warp::reply::with_status("Question addes", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(Error::DatabaseQueryError(err))),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(err) => Err(warp::reject::custom(Error::DatabaseQueryError(err))),
    }
}
