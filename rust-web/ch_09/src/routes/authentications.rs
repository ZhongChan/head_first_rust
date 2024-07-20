use std::future;

use argon2::Config;
use chrono::prelude::*;
use rand::Rng;
use reqwest::StatusCode;
use warp::Filter;

use crate::{
    store::Store,
    types::account::{Account, AccountId, NewAccount, Session},
};

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

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verfied) => {
                if verfied {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => {
                return Err(warp::reject::custom(
                    handle_errors::Error::ArgonLibaryError(e),
                ))
            }
        },
        Err(e) => {
            return Err(warp::reject::custom(e));
        }
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    //token expire
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    let key = "RANDOM WORDS WINTER MACINTOSE PC".as_bytes();
    // local_paseto(&state, None, key)
    //     .expect("Failed to create token")

    let mut builder = paseto::tokens::PasetoBuilder::new();
    builder
        .set_encryption_key(&Vec::from(key))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct a paseto token builder")
}

pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &"RANDOM WORDS WINTER MACINTOSE PC".as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Autorization").and_then(|token| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };
        future::ready(Ok(token))
    })
}
