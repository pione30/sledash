use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
struct EmojiListResponseSchema {
    ok: bool,
    emoji: Option<HashMap<String, String>>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_key = "SLACK_APP_ACCESS_TOKEN";
    let token = env::var(env_key).expect("Error fetching SLACK_APP_ACCESS_TOKEN");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Bearer {}", token).as_str())
            .expect("Bearer token should be a valid header value"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Client should be built");

    let response = client
        .get("https://slack.com/api/emoji.list")
        .send()
        .await
        .unwrap_or_else(|error| {
            let url = error.url().map_or_else(
                || "URL not found.".to_string(),
                |url| url.as_str().to_owned(),
            );

            let status = error.status().map_or_else(
                || "No HTTP status.".to_string(),
                |status| status.as_str().to_owned(),
            );

            panic!("API response error: URL: {}, HTTP status: {}", url, status);
        });

    let response_body = response
        .json::<EmojiListResponseSchema>()
        .await
        .expect("The response body is not in JSON format or it cannot be properly deserialized.");

    if response_body.ok {
        println!("{:#?}", response_body);
    } else {
        panic!(
            "Error is returned from emoji.list API: {}",
            response_body
                .error
                .expect("Some error value should be present when the response_body.ok is false")
        );
    }

    Ok(())
}
