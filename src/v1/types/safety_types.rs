/// Harm categories that would cause prompts or candidates to be blocked.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    HarmCategoryUnspecified,

    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HarmCategoryHateSpeech,

    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    HarmCategorySexuallyExplicit,

    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    HarmCategoryHarassment,

    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    HarmCategoryDangerousContent,
}

/// HarmProbability specifies the probability that a piece of content is harmful.
///
/// The classification system gives the probability of the content being unsafe.
/// This does not indicate the severity of harm for a piece of content.
#[derive(Debug, Clone, serde::Deserialize)]
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

/// Threshold above which a prompt or candidate will be blocked.
#[derive(Debug, Clone, serde::Serialize)]
pub enum HarmBlockThreshold {
    /// Threshold is unspecified.
    // #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified,

    /// Content with NEGLIGIBLE will be allowed.
    // #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,

    /// Content with NEGLIGIBLE and LOW will be allowed.
    // #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,

    /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
    // #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,

    /// All content will be allowed.
    // #[serde(rename = "BLOCK_NONE")]
    BlockNone,
}

/// Safety setting that can be sent as part of request parameters.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}
