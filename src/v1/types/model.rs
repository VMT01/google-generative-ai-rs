use super::{
    content_types::{Content, Tool, ToolConfig},
    generation_types::GenerationConfig,
    safety_types::SafetySetting,
    server::caching::CachedContent,
};

/// Params passed to [GoogleGenerativeAI::get_generative_model][crate::v1::genai::GoogleGenerativeAI::get_generative_model]
#[derive(Debug, Default)]
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The resource name of the `Model`.
    ///
    /// Format: `models/{model}` with a `{model}` naming convention of:
    /// - "{base_model_id}-{version}"
    ///
    /// Examples:
    /// - `models/chat-bison-001`
    pub name: String,

    /// The name of the base model, pass this to the generation request.
    ///
    /// Examples:
    /// - `chat-bison`
    pub base_model_id: Option<String>,

    /// The version number of the model.
    /// This represents the major version
    pub version: String,

    /// The human-readable name of the model. E.g. "Chat Bison".
    /// The name can be up to 128 characters long and can consist of any UTF-8 characters.
    pub display_name: String,

    /// A short description of the model.
    pub description: String,

    /// Maximum number of input tokens allowed for this model.
    pub input_token_limit: u32,

    /// Maximum number of output tokens available for this model.
    pub output_token_limit: u32,

    /// The model's supported generation methods.
    ///
    /// The method names are defined as Pascal case strings, such as `generateMessage` which correspond to API methods.
    pub supported_generation_methods: Vec<String>,

    /// Controls the randomness of the output.
    ///
    /// Values can range over `[0.0,max_temperature]`, inclusive.
    /// A higher value will produce responses that are more varied, while a value closer to `0.0` will typically result in less surprising responses from the model.
    /// This value specifies default to be used by the backend while making the call to the model.
    pub temperature: Option<f32>,

    /// The maximum temperature this model can use.
    pub max_temperature: Option<f32>,

    /// For Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose probability sum is at least `top_p`.
    /// This value specifies default to be used by the backend while making the call to the model.
    pub top_p: Option<f32>,

    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens.
    /// This value specifies default to be used by the backend while making the call to the model.
    /// If empty, indicates the model doesn't use top-k sampling, and `top_k` isn't allowed as a generation parameter.
    pub top_k: Option<u32>,
}
