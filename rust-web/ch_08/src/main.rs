#![warn(clippy::all)]
use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    hyper_request().await?;
    reqwest_request().await?;
    Ok(())
}

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
    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exeact body is send")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", res);

    Ok(())
}
