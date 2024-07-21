use std::{env, future};

use argon2::Config;
use chrono::prelude::*;
use rand::Rng;
use reqwest::StatusCode;
use tracing::Level;
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

    let key = env::var("PASETO_KEY").unwrap();
    println!("PASETO_KEY:{}", key);
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
    tracing::info!(token = token);

    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &Vec::from(key),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|e| {
        tracing::event!(Level::ERROR, "Token validation failed: {:?}", e);
        handle_errors::Error::CannotDecryptToken
    })?;

    serde_json::from_value::<Session>(token).map_err(|e| {
        tracing::event!(
            Level::ERROR,
            "Failed to deserialize token to Session: {:?}",
            e
        );
        handle_errors::Error::CannotDecryptToken
    })
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Autorization").and_then(|token| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(e) => {
                tracing::event!(Level::ERROR, "{:?}", e);
                return future::ready(Err(warp::reject::reject()));
            }
        };
        future::ready(Ok(token))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use paseto::PasetoBuilder;
    use serde_json::json;
    use serial_test::serial;
    use tracing_subscriber;

    fn set_env() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
    }

    #[test]
    #[serial]
    fn test_issue_and_verify_token() {
        set_env();
        // 初始化 tracing
        tracing_subscriber::fmt::try_init().ok();

        // 示例账号 ID
        let account_id = AccountId(4);

        // 生成令牌
        let token = issue_token(account_id.clone());

        // 验证令牌
        let session = verify_token(token.to_string()).expect("Failed to verify token");

        // 检查生成的 session 是否正确
        assert_eq!(session.account_id, account_id);
    }

    #[test]
    #[serial]
    fn test_expired_token() {
        set_env();
        // 初始化 tracing
        tracing_subscriber::fmt::try_init().ok();

        // 示例账号 ID
        let account_id = AccountId(1);

        // 生成过期令牌
        let current_date_time = Utc::now();
        let dt = current_date_time - Duration::days(1); // 令牌过期时间设置为过去
        let key = b"RANDOM WORDS WINTER MACINTOSH PC";
        let mut builder = PasetoBuilder::new();
        let token = builder
            .set_encryption_key(&Vec::from(key))
            .set_expiration(&dt)
            .set_not_before(&Utc::now())
            .set_claim("account_id", json!(account_id))
            .build()
            .expect("Failed to build token");

        // 验证过期令牌应该失败
        let result = verify_token(token);

        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_verify_invalid_token() {
        set_env();
        // 初始化 tracing
        tracing_subscriber::fmt::try_init().ok();

        // 无效的令牌
        let invalid_token = "invalid_token".to_string();

        // 验证令牌应该失败
        let result = verify_token(invalid_token);

        assert!(result.is_err());
    }
}
