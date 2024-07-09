use super::{
    content_types::{Content, Tool, ToolConfig},
    generation_types::GenerationConfig,
    safety_types::SafetySetting,
    server::caching::CachedContent,
};

/// Params passed to [GoogleGenerativeAI::get_generative_model][crate::v1::genai::GoogleGenerativeAI::get_generative_model]
pub struct ModelParams {
    /// The name of the model to query.
    pub model: String,

    /// Sets the default safety filters.
    /// This controls which content is blocked by the api before being returned.
    pub safety_settings: Option<Vec<SafetySetting>>,

    /// A [GenerationConfig] setting the default generation parameters to use.
    pub generation_config: Option<GenerationConfig>,

    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,

    /// SystemInstruction (also known as "system prompt") is a more forceful prompt to the model.
    /// The model will adhere the instructions more strongly than if they appeared in a normal prompt.
    pub system_instruction: Option<Content>,

    /// The name of the CachedContent to use.
    /// Must have already been created with [Client.CreateCachedContent].
    pub cached_content: Option<CachedContent>,
}

impl ModelParams {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            safety_settings: None,
            generation_config: None,
            tools: None,
            tool_config: None,
            system_instruction: None,
            cached_content: None,
        }
    }
}
