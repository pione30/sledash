use reqwest::{header, Client};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_key = "SLACK_APP_ACCESS_TOKEN";
    let token = env::var(env_key).expect("Error fetching SLACK_APP_ACCESS_TOKEN");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
    );

    let client = Client::builder().default_headers(headers).build()?;

    let response = client
        .get("https://slack.com/api/emoji.list")
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", response);
    Ok(())
}
