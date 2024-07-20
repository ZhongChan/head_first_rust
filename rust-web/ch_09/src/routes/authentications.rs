use reqwest::StatusCode;

use crate::{store::Store, types::account::NewAccount};

pub async fn register(
    store: Store,
    account: NewAccount,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
