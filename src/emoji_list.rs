use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

pub async fn fetch(token: &str) -> Result<EmojiListResponse, String> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Bearer {}", token).as_str())
            .map_err(|error| format!("Invalid token was passed: {}", error))?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|error| format!("reqwest client couldn't be built: {}", error))?;

    let response = client
        .get("https://slack.com/api/emoji.list")
        .send()
        .await
        .map_err(|error| {
            let url = error.url().map_or_else(
                || "URL not found.".to_string(),
                |url| url.as_str().to_owned(),
            );

            let status = error.status().map_or_else(
                || "No HTTP status.".to_string(),
                |status| status.as_str().to_owned(),
            );

            format!("API response error: URL: {}, HTTP status: {}", url, status)
        })?;

    response.json::<EmojiListResponse>().await.map_err(|error| {
        format!(
            "the response body is not in JSON format or it cannot be properly deserialized: {}",
            error
        )
    })
}
