use std::collections::HashMap;

use ch_04::QuestionId;
use ch_04::{Error, Pagination, Question, Store};
use warp::filters::body::BodyDeserializeError;
use warp::filters::cors::Builder;
use warp::filters::cors::CorsForbidden;
use warp::http::Method;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

#[tokio::main]
async fn main() {
    //fake database
    let store = Store::new();
    let store_fileter = warp::any().map(move || store.clone());

    // create a path Filter
    let path_hello = warp::path("hello").map(|| warp::reply::html("Hello, Wrap Filter!"));

    // Get first JSON response
    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_fileter.clone())
        .and_then(get_questions);

    // Add question
    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    // Update qestion
    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_fileter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let routes = path_hello
        .or(get_questions)
        .or(add_question)
        .or(update_question)
        .with(get_cors())
        .recover(return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

/// get_cors
/// ```
/// curl -X OPTIONS 127.0.0.1:3030/questions \
/// -H "Access-Control-Request-Method: PUT" \
///-H "Access-Control-Request-Headers: content-type" \
///-H "Origin: https://not-origin.io" \
///--verbose
/// ```
fn get_cors() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST])
}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?; // use ? get Pagination strut
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        // todo out of range
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question addes", StatusCode::OK))
}

async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => {
            return Err(warp::reject::custom(Error::QuestionNotFound));
        }
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("return_error {:?}", r);
    if let Some(error) = r.find::<Error>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ));
    }

    if let Some(error) = r.find::<BodyDeserializeError>() {
        return Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
