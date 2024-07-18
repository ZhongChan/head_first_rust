use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;
use serde::Serialize;
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIResponse {
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsResponse {
    pub content: String,
    #[serde(rename = "bad_words_total")]
    pub bad_words_total: i64,
    #[serde(rename = "bad_words_list")]
    pub bad_words_list: Vec<BadWordsList>,
    #[serde(rename = "censored_content")]
    pub censored_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsList {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    pub start: i64,
    pub end: i64,
    pub replaced_len: i64,
}

pub async fn check_profanity(content: String) -> Result<String, handle_errors::Error> {
    // layerapi bad words
    let api_key: String = env::var("APILAYER_KEY").expect("API_KEY not set");

    //middleware and retry
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(content)
        .send()
        .await
        .map_err(handle_errors::Error::MiddlewareReqwestAPIError)?;

    // transform error
    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ClientError(err));
        } else {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::SereverError(err));
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(err) => Err(handle_errors::Error::ReqwestAPIError(err)),
    }
}

async fn transform_error(resp: reqwest::Response) -> handle_errors::APILayerError {
    handle_errors::APILayerError {
        status: resp.status().as_u16(),
        message: resp.json::<APIResponse>().await.unwrap().message,
    }
}
