use std::collections::HashMap;

use crate::v1::traits::Stringify;

use super::{
    content_types::{Content, Tool, ToolConfig},
    generation_types::GenerationConfig,
    safety_types::SafetySetting,
    server::caching::CachedContent,
};

/// Params passed to getGenerativeModel() or GoogleAIFileManager().
#[derive(Debug)]
pub struct RequestOptions {
    /// Request timeout in milliseconds.
    pub timeout: Option<u64>,

    /// Version of API endpoint to call (e.g. "v1" or "v1beta").
    /// If not specified, defaults to latest stable version.
    pub api_version: Option<ApiVersion>,

    /// Additional attribution information to include in the x-goog-api-client header.
    /// Used by wrapper SDKs.
    pub api_client: Option<String>,

    /// Base endpoint url. Defaults to "https://generativelanguage.googleapis.com"
    pub base_url: Option<String>,

    /// Custom HTTP request headers.
    pub custom_headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Clone)]
pub enum ApiVersion {
    V1,
    #[default]
    V1Beta,
}

impl Stringify for ApiVersion {
    fn to_str(&self) -> &'static str {
        match self {
            Self::V1 => "v1",
            Self::V1Beta => "v1beta",
        }
    }
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            timeout: None,
            api_version: Some(ApiVersion::default()),
            api_client: None,
            base_url: Some(String::from("https://generativelanguage.googleapis.com")),
            custom_headers: None,
        }
    }
}

/// GenerateContentRequest: Request to generate a completion from the model.
#[derive(Debug, Default, serde::Serialize)]
pub struct GenerateContentRequest {
    /// The name of the `Model` to use for generating the completion.
    /// Format: `name=models/{model}`.
    pub model: String,

    /// The content of the current conversation with the model.
    /// For single-turn queries, this is a single instance.
    /// For multi-turn queries, this is a repeated field that contains conversation history + latest request.
    pub contents: Vec<Content>,

    /// Configuration options for model generation and outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,

    /// A list of unique `SafetySetting` instances for blocking unsafe content.
    /// This will be enforced on the [GenerateContentRequest::contents] and [GenerateContentResponse::candidates][crate::v1::types::responses::GenerateContentResponse::candidates].
    /// There should not be more than one setting for each [SafetyCategory][crate::v1::types::safety_types::SafetySetting::category] type.
    /// The API will block any contents and responses that fail to meet the thresholds set by these settings.
    /// This list overrides the default settings for each [SafetyCategory][crate::v1::types::safety_types::SafetySetting::category] specified in the safety_settings.
    /// If there is no `SafetySetting` for a given `SafetyCategory` provided in the list, the API will use the default safety setting for that category.
    /// Harm categories HARM_CATEGORY_HATE_SPEECH, HARM_CATEGORY_SEXUALLY_EXPLICIT, HARM_CATEGORY_DANGEROUS_CONTENT, HARM_CATEGORY_HARASSMENT are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,

    /// A list of `Tools` the model may use to generate the next response.
    /// A `Tool` is a piece of code that enables the system to interact with external systems to perform an action, or set of actions, outside of knowledge and scope of the model.
    /// The only supported tool is currently `Function`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Tool configuration for any `Tool` specified in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,

    /// Developer set system instruction. Currently, text only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,

    /// The name of the cached content used as context to serve the prediction.
    /// Note: only used in explicit caching, where users can have control over caching (e.g. what content to cache) and enjoy guaranteed cost savings.
    /// Format: `cachedContents/{cachedContent}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<CachedContent>,
}

pub enum Task {
    GenerateContent,
    StreamGenerateContent,
    CountTokens,
    EmbedContent,
    BatchEmbedContents,
}

impl Stringify for Task {
    fn to_str(&self) -> &'static str {
        match self {
            Self::GenerateContent => "generateContent",
            Self::StreamGenerateContent => "streamGenerateContent",
            Self::CountTokens => "countTokens",
            Self::EmbedContent => "embedContent",
            Self::BatchEmbedContents => "batchEmbedContents",
        }
    }
}
