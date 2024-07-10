use anyhow::{Ok, Result};

use super::{
    traits::Stringify,
    types::{requests::RequestOptions, responses::ListModelResponse},
};

pub mod generative_models;

pub async fn list_models(
    api_key: String,
    request_options: Option<RequestOptions>,
) -> Result<ListModelResponse> {
    let request_options = request_options.unwrap_or_default();
    let api_version = request_options.api_version.unwrap_or_default().to_str();
    let base_url = request_options.base_url.unwrap_or_default();

    let url = format!("{base_url}/{api_version}/models?key={api_key}");
    let response = reqwest::Client::new().get(url).send().await?;

    Ok(response.json().await.unwrap())
}
