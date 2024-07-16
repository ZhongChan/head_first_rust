use crate::{
    store::Store,
    types::{
        answer::{Answer, AnswerId},
        question::QuestionId,
    },
};
use std::collections::HashMap;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

/// For Anwsers
pub async fn add_answer(
    store: Store,
    params: HashMap<String, i32>,
) -> Result<impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(*params.get("questionId").unwrap()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
