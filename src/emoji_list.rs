use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

pub async fn fetch(token: &str) -> EmojiListResponse {
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

    response
        .json::<EmojiListResponse>()
        .await
        .expect("The response body should be in JSON format or be properly deserialized")
}
