use super::common::{ResourceId, ResourceName};
use serde::{Deserialize, Serialize};

/// Source resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Full resource name
    pub name: ResourceName,

    /// Output only. Source ID
    pub id: ResourceId,

    /// GitHub repo
    #[serde(rename = "githubRepo", skip_serializing_if = "Option::is_none")]
    pub github_repo: Option<GitHubRepo>,
}

/// GitHub repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    /// Repository owner
    pub owner: String,

    /// Repository name
    pub repo: String,

    /// Is private repository
    #[serde(rename = "isPrivate", skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    /// Default branch
    #[serde(rename = "defaultBranch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<GitHubBranch>,

    /// All branches
    #[serde(default)]
    pub branches: Vec<GitHubBranch>,
}

/// GitHub branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubBranch {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

/// List sources response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSourcesResponse {
    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
