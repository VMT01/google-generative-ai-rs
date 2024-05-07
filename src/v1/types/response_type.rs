use super::{content_types::ContentType, safety_types::HarmCategory};

/// Individual response from [generate_content][crate::v1::generative_models::GenerativeModel::generate_content]
/// and [generate_content_stream][crate::v1::generative_models::GenerativeModel::generate_content_stream].
/// `generate_content_stream` will return one in each chunk until the stream is done.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    pub candidates: Vec<GenerateContentCandidate>,
    // The prompt's feedback related to the content filters.
    // pub prompt_feedback: Option<PromptFeedback>,

    // Metadata on the generation request's token usage.
    // pub usage_metadata: Option<UsageMetadata>,
}

/// A candidate returned as part of a [GenerateContentResponse].
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentCandidate {
    pub index: i32,
    pub content: ContentType,
    pub finish_reason: Option<FinishReason>,
    pub finish_message: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    pub citation_metadata: Option<CitationMetadata>,
}

/// Reason that a candidate finished.
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

    /// Unknown reason.
    #[serde(rename = "OTHER")]
    Other,
}

/// A safety rating associated with a [GenerateContentCandidate]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

/// Probability that a prompt or candidate matches a harm category.
#[derive(Debug, serde::Deserialize)]
pub enum HarmProbability {
    /// Probability is unspecified.
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    HarmProbabilityUnspecified,

    /// Content has a negligible chance of being unsafe.
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,

    /// Content has a low chance of being unsafe.
    #[serde(rename = "LOW")]
    Low,

    /// Content has a medium chance of being unsafe.
    #[serde(rename = "MEDIUM")]
    Medium,

    /// Content has a high chance of being unsafe.
    #[serde(rename = "HIGH")]
    High,
}

/// Citation metadata that may be found on a [GenerateContentCandidate]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    pub citation_sources: Vec<CitationSource>,
}

/// A single citation source.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    pub start_index: Option<i32>,
    pub end_index: Option<i32>,
    pub uri: Option<String>,
    pub license: Option<String>,
}

/// If the prompt was blocked, this will be populated with [PromptFeedback::block_reason] and the relevant [PromptFeedback::safety_ratings].
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    pub block_reason: BlockReason,
    pub safety_ratings: Vec<SafetyRating>,
    pub block_reason_message: Option<String>,
}

/// Reason that a prompt was blocked.
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

/// Metadata on the generation request's token usage.
#[derive(Debug, serde::Deserialize)]
pub struct UsageMetadata {
    /// Number of tokens in the prompt.
    pub prompt_token_count: i32,

    /// Total number of tokens across the generated candidates.
    pub candidates_token_count: i32,

    /// Total token count for the generation request (prompt + candidates).
    pub total_token_count: i32,
}
