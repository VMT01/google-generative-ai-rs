use std::collections::HashMap;

/// Params passed to getGenerativeModel() or GoogleAIFileManager().
#[derive(Debug)]
pub struct RequestOptions {
    /// Request timeout in milliseconds.
    pub timeout: Option<i32>,

    /// Version of API endpoint to call (e.g. "v1" or "v1beta").
    /// If not specified, defaults to latest stable version.
    pub api_version: Option<String>,

    /// Additional attribution information to include in the
    /// x-goog-api-client header. Used by wrapper SDKs.
    pub api_client: Option<String>,

    /// Base endpoint url. Defaults to "https://generativelanguage.googleapis.com"
    pub base_url: Option<String>,

    /// Custom HTTP request headers.
    pub custom_headers: Option<HashMap<String, String>>,
}
