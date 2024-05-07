use super::types::{
    content_types::{ContentType, Tool, ToolConfig},
    generation_types::GenerationConfig,
    request_types::RequestOptions,
    safety_types::SafetySetting,
};

/// The [GenerativeModel] class wraps default parameters for calls to
/// [GenerativeModel::generate_content], [GenerativeModel::count_tokens]
/// and [GenerativeModel::start_chat].
///
/// This family of functionality is designed to support multi-turn
/// conversations, and multimodal requests. What media-types are
/// supported for input and output is model-dependant.
// TODO: Add simple example
#[derive(Debug)]
pub struct GenerativeModel {
    api_key: String,
    /// Currently support models/gemini-pro & models/gemini-pro-vision
    pub model_name: String,
    pub safety_settings: Option<Vec<SafetySetting>>,
    pub generation_config: Option<GenerationConfig>,
    pub tools: Option<Vec<Tool>>,
    pub tool_config: Option<ToolConfig>,
    pub system_instruction: Option<ContentType>,
    pub request_options: Option<RequestOptions>,
}

impl GenerativeModel {
    /// Gets a [GenerativeModel] instance for the provided model name
    pub fn new(
        api_key: String,
        model_name: String,
        safety_settings: Option<Vec<SafetySetting>>,
        generation_config: Option<GenerationConfig>,
        tools: Option<Vec<Tool>>,
        tool_config: Option<ToolConfig>,
        system_instruction: Option<ContentType>,
        request_options: Option<RequestOptions>,
    ) -> Self {
        Self {
            api_key,
            model_name,
            safety_settings,
            generation_config,
            tools,
            tool_config,
            system_instruction,
            request_options,
        }
    }

    /// A multipurpose function to generate responses from the model.
    ///
    /// This [GenerativeModel::generate_content] method can handle multimodal
    /// input, and multi-turn conversations.
    ///
    /// ```rs                            TODO: Add demo code
    /// let model = GenerativeModel::new(
    ///     "abcxyz".to_string(),
    ///     "models/gemini-pro".to_string(),
    ///     None, None, None, None, None, None
    /// );
    /// let response = model.generate_content(
    ///     ContentType {
    ///         role: "user",
    ///         parts: vec![Part::TextPart(TextPart {
    ///             text: "Tell me a story about a magic backpack"
    ///         })]
    ///     }
    /// );
    /// ```
    ///
    /// Arguments
    /// - `contents`: The contents serving as the model's prompt.
    /// - `system_instruction`: The contents serving as the model's instruction.
    /// - `generation_config`: Overrides for the model's generation config.
    /// - `safety_settings`: Overrides for the model's safety settings.
    /// - `stream`: If True, yield response chunks as they are generated.
    /// - `tools`: Wating from Google Generative
    /// - `tool_config`: Wating from Google Generative
    /// - `request_options`: Options for the request.
    pub fn generate_content(
        self,
        contents: Vec<ContentType>,
        system_instruction: Option<ContentType>,
        generation_config: Option<GenerationConfig>,
        safety_settings: Option<Vec<SafetySetting>>,
        stream: bool,
        tools: Option<Vec<Tool>>,
        tool_config: Option<ToolConfig>,
        request_options: Option<RequestOptions>,
    ) {
    }

    // pub fn generate_content_stream() {}

    // pub async fn generate_content_async() {}

    // pub fn count_tokens() {}

    // pub fn start_chat() {}
}

// mod tests {
//     #[test]
//     fn
// }
