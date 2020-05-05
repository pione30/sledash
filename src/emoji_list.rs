use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmojiListError {
    #[error("invalid token was passed: {0}")]
    InvalidTokenPassed(header::InvalidHeaderValue),
    #[error("reqwest client couldn't be built: {0}")]
    ClientBuildFailed(reqwest::Error),
    #[error("API request failed: URL: {url}, HTTP status: {status}")]
    APIRequestFailed { url: String, status: String },
    #[error("the response body is not in JSON format or it cannot be properly deserialized: {0}")]
    ResponseUndeserializable(reqwest::Error),
}

#[derive(Deserialize, Debug)]
pub struct EmojiListResponse {
    pub ok: bool,
    pub emoji: Option<HashMap<String, String>>,
    pub error: Option<String>,
}

pub async fn fetch(token: &str) -> Result<EmojiListResponse, EmojiListError> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(format!("Bearer {}", token).as_str())
            .map_err(EmojiListError::InvalidTokenPassed)?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(EmojiListError::ClientBuildFailed)?;

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

            EmojiListError::APIRequestFailed { url, status }
        })?;

    response
        .json::<EmojiListResponse>()
        .await
        .map_err(EmojiListError::ResponseUndeserializable)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn invalid_token_error() {
        let invalid_token = "invalid!\n";
        let result = fetch(invalid_token).await;
        assert!(result.is_err());
    }
}
