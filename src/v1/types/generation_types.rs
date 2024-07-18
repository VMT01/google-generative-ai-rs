use super::schema::Schema as ResponseSchema;

/// GenerationConfig is configuration options for model generation and outputs.
/// Not all parameters may be configurable for every model.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct GenerationConfig {
    /// Number of generated responses to return.
    pub candidate_count: Option<u32>,

    /// The set of character sequences (up to 5) that will stop output generation.
    /// If specified, the API will stop at the first appearance of a stop sequence.
    /// The stop sequence will not be included as part of the response.
    pub stop_sequences: Option<Vec<String>>,

    /// The maximum number of tokens to include in a candidate.
    /// If unset, this will default to output_token_limit specified in the model's specification.
    pub max_output_tokens: Option<u128>,

    /// Controls the randomness of the output.
    /// Note: The default value varies by model, see the `Model.temperature` attribute of the `Model` returned the `genai.get_model` function.
    ///
    /// Values can range from [0.0, 1.0], inclusive.
    /// A value closer to 1.0 will produce responses that are more varied and creative,
    /// while a value closer to 0.0 will typically result in more straightforward responses from the model.
    pub temperature: Option<f32>,

    /// The maximum cumulative probability of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so that only the most likely tokens are considered.
    /// Top-k sampling directly limits the maximum number of tokens to consider,
    /// while Nucleus sampling limits number of tokens based on the cumulative probability.
    ///
    /// Note: The default value varies by model, see the `Model.top_p` attribute of the `Model` returned the `genai.get_model` function.
    pub top_p: Option<f32>,

    /// The maximum number of tokens to consider when sampling.
    ///
    /// The model uses combined Top-k and nucleus sampling.
    ///
    /// Top-k sampling considers the set of `top_k` most probable tokens. Defaults to 40.
    /// Note: The default value varies by model, see the `Model.top_k` attribute of the `Model` returned the `genai.get_model` function.
    pub top_k: Option<f32>,

    /// Output response mimetype of the generated candidate text.
    /// Supported mimetype:
    /// - `text/plain`: (default) Text output.
    /// - `application/json`: JSON response in the candidates.
    pub response_mime_type: Option<MimeType>,

    pub response_schema: Option<ResponseSchema>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum MimeType {
    // #[serde(rename = "text/plain")]
    TextPlain,
    // #[serde(rename = "application/json")]
    ApplicationJson,
}
