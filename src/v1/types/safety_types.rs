/// Safety setting that can be sent as part of request parameters.
#[derive(Debug)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

/// Harm categories that would cause prompts or candidates to be blocked.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

/// Threshold above which a prompt or candidate will be blocked.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum HarmBlockThreshold {
    /// Threshold is unspecified.
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified,

    /// Content with NEGLIGIBLE will be allowed.
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,

    /// Content with NEGLIGIBLE and LOW will be allowed.
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,

    /// Content with NEGLIGIBLE, LOW, and MEDIUM will be allowed.
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,

    /// All content will be allowed.
    #[serde(rename = "BLOCK_NONE")]
    BlockNone,
}
