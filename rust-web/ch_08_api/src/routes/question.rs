use crate::{
    store::Store,
    types::{
        apilayer::{APIResponse, BadWordsResponse},
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
    },
};

use std::collections::HashMap;
use std::env;
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
    // layerapi bad words
    let api_key: String = env::var("APILAYER_KEY").expect("API_KEY not set");
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(new_question.content.clone())
        .send()
        .await
        .map_err(handle_errors::Error::ExternalAPIError)?;

    // transform error
    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(warp::reject::custom(handle_errors::Error::ClientError(err)));
        } else {
            let err = transform_error(res).await;
            return Err(warp::reject::custom(handle_errors::Error::SereverError(
                err,
            )));
        }
    }

    // parse json
    let res = res
        .json::<BadWordsResponse>()
        .await
        .map_err(handle_errors::Error::ExternalAPIError)?;

    let content = res.censored_content;
    let question = NewQuestion {
        title: new_question.title,
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

async fn transform_error(resp: reqwest::Response) -> handle_errors::APILayerError {
    handle_errors::APILayerError {
        status: resp.status().as_u16(),
        message: resp.json::<APIResponse>().await.unwrap().message,
    }
}
