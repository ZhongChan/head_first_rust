use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;
use serde::Serialize;
use std::env;
use tracing::Level;

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
    let api_key: String = env::var("APILAYER_KEY").expect("APILAYER KEY not set");

    //middleware and retry
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let apilayer_url = env::var("APILAYER_URL").expect("APILAYER URL NOT SET");
    let res = client
        .post(format!("{}/bad_words?censor_character=*", apilayer_url))
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
    // 提取响应状态码
    let status = resp.status().as_u16();

    // 提取响应体并转换为字符串
    let body = resp
        .text()
        .await
        .unwrap_or_else(|_| "Failed to read response body".to_string());

    // 使用 tracing 记录响应体
    tracing::event!(Level::INFO, "Response body: {}", body);

    // 假设 APIResponse 结构体包含一个 message 字段
    let message = if let Ok(api_response) = serde_json::from_str::<APIResponse>(&body) {
        api_response.message
    } else {
        "Failed to parse JSON response".to_string()
    };

    handle_errors::APILayerError { status, message }
}

#[cfg(test)]
mod apilayer_tests {
    use super::{check_profanity, env};
    use mock_server::{MockServer, OneshotHandler};

    #[tokio::test]
    async fn run() {
        let handler = run_mock();
        censor_profane_words().await;
        no_profane_words().await;
        let _ = handler.sender.send(1); //close server
    }

    fn run_mock() -> OneshotHandler {
        env::set_var("APILAYER_URL", "http://127.0.0.1:3030");
        env::set_var("APILAYER_KEY", "YES");

        let socket = "127.0.0.1:3030"
            .to_string()
            .parse()
            .expect("Not a valid address");
        let mock = MockServer::new(socket);
        mock.oneshot()
    }

    async fn censor_profane_words() {
        let content = "This is a shitty sentence".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(censored_content.unwrap(), "sdfsd")
    }

    async fn no_profane_words() {
        let content = "This is a shitty sentence".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(censored_content.unwrap(), "sdfsd")
    }
}
