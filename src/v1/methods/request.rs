use std::fmt::Display;

use crate::v1::{
    constants::request::{DEFAULT_API_VERSION, DEFAULT_BASE_URL},
    types::{request_types::RequestOptions, response_type::GenerateContentResponse},
};

#[derive(Debug)]
pub enum Task {
    GenerateContent,
    StreamGenerateContent,
    CountTokens,
    EmbedContent,
    BatchEmbedContents,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Task::GenerateContent => f.write_str("generateContent"),
            Task::StreamGenerateContent => f.write_str("streamGenerateContent"),
            Task::CountTokens => f.write_str("countTokens"),
            Task::EmbedContent => f.write_str("embedContent"),
            Task::BatchEmbedContents => f.write_str("batchEmbedContents"),
        }
    }
}

pub async fn make_request(
    model: String,
    task: Task,
    api_key: String,
    stream: bool,
    body: String,
    request_options: Option<RequestOptions>,
) -> Result<GenerateContentResponse, Box<dyn std::error::Error>> {
    let base_url = request_options
        .as_ref()
        .map_or(DEFAULT_BASE_URL.to_string(), |options| {
            options
                .base_url
                .clone()
                .unwrap_or(DEFAULT_BASE_URL.to_string())
        });

    let api_version = request_options
        .as_ref()
        .map_or(DEFAULT_API_VERSION.to_string(), |options| {
            options
                .api_version
                .clone()
                .unwrap_or(DEFAULT_API_VERSION.to_string())
        });

    let stream = if stream { "&alt=sse" } else { "" };

    let url = format!(
        "{}/{}/{}:{}?key={}{}",
        base_url, api_version, model, task, api_key, stream
    );

    let response = reqwest::Client::new()
        .post(url)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await?;
    let data = response.json::<GenerateContentResponse>().await?;
    Ok(data)
}

mod tests {
    #[tokio::test]
    async fn test_make_request() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("GOOGLE_API_KEY").unwrap();

        let response = super::make_request(
            "models/gemini-pro".to_string(),
            super::Task::GenerateContent,
            api_key,
            false,
            "{\"contents\":[{\"parts\":[{\"text\":\"Hello.\"}]}]}".to_string(),
            None,
        )
        .await;

        assert!(response.is_ok());
        dbg!(response.unwrap());
    }
}
