use argon2::Config;
use rand::Rng;
use reqwest::StatusCode;

use crate::{store::Store, types::account::NewAccount};

pub async fn register(
    store: Store,
    account: NewAccount,
) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());
    let account = NewAccount {
        nickname: account.nickname,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
