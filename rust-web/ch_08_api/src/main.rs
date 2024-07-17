use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("APILAYER_KEY").expect("API_KEY not set");

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
