use crate::{apilayer, store::Store, types::answer::NewAnswer};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

/// For Anwsers
pub async fn add_answer(store: Store, new_answer: NewAnswer) -> Result<impl Reply, Rejection> {
    let content = match apilayer::check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(err) => return Err(warp::reject::custom(err)),
    };

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    if let Err(err) = store.add_answer(answer).await {
        return Err(warp::reject::custom(err));
    }
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
