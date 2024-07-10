use super::{
    content_types::Content,
    model::Model,
    safety_types::{HarmCategory, HarmProbability},
};

/// GenerateContentResponse is the response from a [generate_content][crate::v1::models::generative_models::GenerativeModel::generate_content] or GenerateContentStream call.
///
/// Note on safety ratings and content filtering.
/// They are reported for both prompt in [GenerateContentResponse::prompt_feedback] and for each candidate in `finish_reason` and in `safety_ratings`.
/// The API contract is that:
/// - either all requested candidates are returned or no candidates at all
/// - no candidates are returned only if there was something wrong with the prompt (see `prompt_feedback`)
/// - feedback on each candidate is reported on `finish_reason` and `safety_ratings`.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,

    /// Returns the prompt's feedback related to the content filters.
    pub prompt_feedback: Option<PromptFeedback>,
    /// Output only. Metadata on the generation requests' token usage.
    pub usage_metadata: UsageMetadata,
}

/// Candidate is a response candidate generated from the model.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// Output only. Index of the candidate in the list of candidates.
    pub index: u32,

    /// Output only. Generated content returned from the model.
    pub content: Content,

    /// Output only. The reason why the model stopped generating tokens.
    ///
    /// If empty, the model has not stopped generating the tokens.
    pub finish_reason: Option<FinishReason>,

    /// List of ratings for the safety of a response candidate.
    ///
    /// There is at most one rating per category.
    pub safety_ratings: Vec<SafetyRating>,

    /// Output only. Citation information for model-generated candidate.
    ///
    /// This field may be populated with recitation information for any text included in the `content`.
    /// These are passages that are "recited" from copyrighted material in the foundational LLM's training data.
    pub citation_metadata: Option<CitationMetadata>,
    pub token_count: Option<u32>,
}

/// FinishReason is defines the reason why the model stopped generating tokens.
#[derive(Debug, serde::Deserialize)]
pub enum FinishReason {
    /// Default value. This value is unused.
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,

    /// Natural stop point of the model or provided stop sequence.
    #[serde(rename = "STOP")]
    Stop,

    /// The maximum number of tokens as specified in the request was reached.
    #[serde(rename = "MAX_TOKENS")]
    MaxTokens,

    /// The candidate content was flagged for safety reasons.
    #[serde(rename = "SAFETY")]
    Safety,

    /// The candidate content was flagged for recitation reasons.
    #[serde(rename = "RECITATION")]
    Recitation,

    /// The candidate content was flagged for using an unsupported language.
    #[serde(rename = "LANGUAGE")]
    Language,

    /// Unknown reason.
    #[serde(rename = "OTHER")]
    Other,
}

/// SafetyRating is the safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of harm categories and the probability of the harm classification is included here.
#[derive(Debug, serde::Deserialize)]
pub struct SafetyRating {
    /// The category for this rating.
    pub category: HarmCategory,

    /// The probability of harm for this content.
    pub probability: HarmProbability,

    /// Was this content blocked because of this rating?
    pub blocked: Option<bool>,
}

/// CitationMetadata is a collection of source attributions for a piece of content.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    pub citation_sources: Vec<CitationSource>,
}

/// CitationSource contains a citation to a source for a portion of a specific response.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    /// Start of segment of the response that is attributed to this source.
    ///
    /// Index indicates the start of the segment, measured in bytes.
    pub start_index: Option<u32>,

    /// End of the attributed segment, exclusive.
    pub end_index: Option<u32>,

    /// URI that is attributed as a source for a portion of the text.
    pub uri: Option<String>,

    /// License for the GitHub project that is attributed as a source for segment.
    ///
    /// License info is required for code citations.
    pub license: Option<String>,
}

/// PromptFeedback contains a set of the feedback metadata the prompt specified in [GenerateContentRequest::contents][crate::v1::types::requests::GenerateContentRequest::contents]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    /// The prompt was blocked and no candidates are returned. Rephrase your prompt.
    pub block_reason: BlockReason,

    /// Ratings for safety of the prompt.
    /// There is at most one rating per category.
    pub safety_ratings: Vec<SafetyRating>,

    pub block_reason_message: Option<String>,
}

/// BlockReason is specifies what was the reason why prompt was blocked.
#[derive(Debug, serde::Deserialize)]
pub enum BlockReason {
    /// A blocked reason was not specified.
    #[serde(rename = "BLOCKED_REASON_UNSPECIFIED")]
    BlockedReasonUnspecified,

    /// Content was blocked by safety settings.
    #[serde(rename = "SAFETY")]
    Safety,

    /// Content was blocked, but the reason is uncategorized.
    #[serde(rename = "OTHER")]
    Other,
}

/// UsageMetadata is metadata on the generation request's token usage.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    /// Number of tokens in the prompt.
    /// When cached_content is set, this is still the total effective prompt size.
    /// I.e. this includes the number of tokens in the cached content.
    pub prompt_token_count: u32,

    /// Total number of tokens across the generated candidates.
    pub candidates_token_count: u32,

    /// Total token count for the generation request (prompt + candidates).
    pub total_token_count: u32,

    /// Number of tokens in the cached part of the prompt, i.e. in the cached content.
    pub cached_content_token_count: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListModelResponse {
    pub models: Vec<Model>,
}
