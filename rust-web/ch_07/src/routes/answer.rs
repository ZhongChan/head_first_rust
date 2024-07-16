use crate::{store::Store, types::answer::NewAnswer};
use handle_errors::Error;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

/// For Anwsers
pub async fn add_answer(store: Store, new_answer: NewAnswer) -> Result<impl Reply, Rejection> {
    if let Err(err) = store.add_answer(new_answer).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(err)));
    }
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
