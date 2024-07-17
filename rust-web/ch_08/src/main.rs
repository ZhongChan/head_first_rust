#![warn(clippy::all)]
use hyper::{body::HttpBody as _, Client};
use std::env;
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // hyper_request().await?;
    reqwest_request().await?;
    Ok(())
}

#[warn(dead_code)]
async fn hyper_request() -> Result<()> {
    let client = Client::new();

    let mut res = client
        .get("http://www.baidu.com".parse::<hyper::Uri>().unwrap())
        .await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }
    println!("\n\nDone!");
    Ok(())
}

async fn reqwest_request() -> Result<()> {
    let api_key: String = env::var("APILAYER_KEY").expect("API_KEY not set");

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body("a list whith shit words")
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", res);
    Ok(())
}
