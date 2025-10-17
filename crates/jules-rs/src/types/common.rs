use serde::{Deserialize, Serialize};

/// Pagination response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    #[serde(flatten)]
    pub items: T,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

/// Timestamp wrapper (RFC 3339)
pub type Timestamp = String;

/// Resource name (e.g., "sessions/123")
pub type ResourceName = String;

/// Resource ID (e.g., "123")
pub type ResourceId = String;
