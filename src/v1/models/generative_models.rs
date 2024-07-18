use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Response;

use crate::v1::{
    traits::Stringify,
    types::{
        content_types::{Content, Part, Role, Tool, ToolConfig},
        generation_types::GenerationConfig,
        model::ModelParams,
        requests::{GenerateContentRequest, RequestOptions, Task},
        responses::GenerateContentResponse,
        safety_types::SafetySetting,
        server::caching::CachedContent,
    },
};

/// GenerativeModel is a model that can generate text.
#[derive(Debug)]
pub struct GenerativeModel {
    api_key: String,
    pub model: String,
    pub request_options: RequestOptions,
    pub generation_config: Option<GenerationConfig>,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,
    pub system_instruction: Option<Content>,
    pub cached_content: Option<CachedContent>,
}

impl GenerativeModel {
    pub fn new(
        api_key: String,
        model_params: ModelParams,
        request_options: Option<RequestOptions>,
    ) -> Self {
        let model = if model_params.model.contains('/') {
            model_params.model
        } else {
            format!("models/{}", model_params.model)
        };

        Self {
            api_key,
            model,
            generation_config: model_params.generation_config,
            safety_settings: model_params.safety_settings,
            request_options: request_options.unwrap_or_default(),
            tools: model_params.tools,
            tool_config: model_params.tool_config,
            system_instruction: model_params.system_instruction,
            cached_content: model_params.cached_content,
        }
    }

    /// Create a [GenerateContentRequest][crate::v1::types::requests::GenerateContentRequest] from raw inputs
    fn _prepare_request(&self, requests: Vec<Part>) -> GenerateContentRequest {
        GenerateContentRequest {
            model: self.model.clone(),
            contents: vec![Content {
                role: Role::User,
                parts: requests,
            }],
            generation_config: self.generation_config.clone(),
            safety_settings: self.safety_settings.clone(),
            tools: self.tools.clone(),
            tool_config: self.tool_config.clone(),
            system_instruction: self.system_instruction.clone(),
            cached_content: self.cached_content.clone(),
        }
    }

    async fn _make_model_request(
        &self,
        task: Task,
        params: GenerateContentRequest,
        stream: bool,
    ) -> Result<Response> {
        let api_version = self
            .request_options
            .api_version
            .as_ref()
            .map(|v| v.to_str())
            .unwrap_or_default();
        let base_url = self.request_options.base_url.as_deref().unwrap_or_default();
        let mut url = format!(
            "{}/{}/{}:{}?key={}",
            base_url,
            api_version,
            self.model,
            task.to_str(),
            self.api_key,
        );
        if stream {
            url.push_str("&alt=sse");
        }

        let body = serde_json::to_string(&params)?;
        let response = reqwest::Client::new()
            .post(&url)
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await?;

        Ok(response)
    }

    /// A multipurpose function to generate responses from the model.
    ///
    /// This `GenerativeModel.generate_content` method can handle multimodal input, and multi-turn conversations.
    ///
    /// ```
    /// use google_generative_ai_rs::v1::genai::GoogleGenerativeAI;
    /// use google_generative_ai_rs::v1::types::model::ModelParams;
    ///
    /// let api_key = "".to_string();
    /// let genai = GoogleGenerativeAI::new(api_key)
    ///     .get_generative_model(ModelParams::new("gemini-1.5-flash"), None);
    /// let response = genai
    ///     .generate_content(vec![google_generative_ai_rs::v1::types::content_types::Part {
    ///         text: Some("Hello Gemini".to_string()),
    ///         ..Default::default()
    ///     }]);
    /// ```
    pub async fn generate_content(&self, requests: Vec<Part>) -> Result<GenerateContentResponse> {
        let content = self._prepare_request(requests);
        let response = self
            ._make_model_request(Task::GenerateContent, content, false)
            .await?;
        let content_response = response.json().await?;
        Ok(content_response)
    }

    pub async fn generate_content_stream(&self, requests: Vec<Part>) -> Result<()> {
        let content = self._prepare_request(requests);
        let response = self
            ._make_model_request(Task::GenerateContent, content, true)
            .await?;

        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item?;
            let str = std::str::from_utf8(&item)?;
            for p in str.split("\n\n") {
                dbg!(p);
            }
        }

        Ok(())
    }
}
