use crate::v1::types::content_types::{Content, Tool, ToolConfig};

/// CachedContent is content that has been preprocessed and can be used in subsequent request to GenerativeService.
/// Cached content can be only used with model it was created for.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CachedContent {
    /// Specifies when this resource will expire.
    /// Types that are assignable to Expiration:
    /// - CachedContent_ExpireTime
    /// - CachedContent_Ttl
    pub expiration: ExpireTimeOrTTL,

    /// Identifier. The resource name referring to the cached content.
    /// Format: `cachedContents/{id}`
    pub name: Option<String>,

    /// Immutable. The user-generated meaningful display name of the cached content. Maximum 128 Unicode characters.
    pub display_name: Option<String>,

    /// Immutable. The name of the `Model` to use for cached content
    /// Format: `models/{model}`
    pub model: String,

    /// Input only. Immutable. Developer set system instruction.
    /// Currently text only.
    pub system_instruction: Content,

    /// Input only. Immutable. Developer set system instruction.
    /// Currently text only.
    pub contents: Vec<Content>,

    /// Input only. Immutable. A list of `Tools` the model may use to generate the next response
    pub tools: Option<Vec<Tool>>,

    /// Input only. Immutable. Tool config. This config is shared for all tools.
    pub tool_config: Option<ToolConfig>,

    /// Output only. Creation time of the cache entry.
    pub create_time: String,

    /// Output only. When the cache entry was last updated in UTC time.
    pub update_time: String,

    /// Output only. Metadata on the usage of the cached content.
    pub usage_metadata: CachedContentUsageMetadata,
}

/// ExpireTimeOrTTL describes the time when a resource expires.
/// If expire_time is non-zero, it is the expiration time.
/// Otherwise, the expiration time is the value of TTL ("time to live") added to the current time.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExpireTimeOrTTL {
    pub expire_time: String,
    pub ttl: String,
}

/// CachedContentUsageMetadata is metadata on the usage of the cached content.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CachedContentUsageMetadata {
    /// Total number of tokens that the cached content consumes.
    pub total_token_count: u32,
}
