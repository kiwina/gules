use super::common::{ResourceId, ResourceName, Timestamp};
use serde::{Deserialize, Serialize};

/// Session resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Output only. Full resource name
    pub name: ResourceName,

    /// Output only. Session ID
    pub id: ResourceId,

    /// Required. Initial prompt
    pub prompt: String,

    /// Required. Source context
    #[serde(rename = "sourceContext")]
    pub source_context: SourceContext,

    /// Optional. Session title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Optional. Input only. Require plan approval
    #[serde(
        rename = "requirePlanApproval",
        skip_serializing_if = "Option::is_none"
    )]
    pub require_plan_approval: Option<bool>,

    /// Optional. Input only. Automation mode
    #[serde(rename = "automationMode", skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,

    /// Output only. Creation time
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<Timestamp>,

    /// Output only. Last update time
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<Timestamp>,

    /// Output only. Session state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,

    /// Output only. Web app URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Output only. Session outputs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<SessionOutput>,
}

/// Session state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    StateUnspecified,
    Queued,
    Planning,
    AwaitingPlanApproval,
    AwaitingUserFeedback,
    InProgress,
    Paused,
    Failed,
    Completed,
}

impl State {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            State::StateUnspecified => "Unspecified",
            State::Queued => "Queued",
            State::Planning => "Planning",
            State::AwaitingPlanApproval => "Awaiting Plan Approval",
            State::AwaitingUserFeedback => "Awaiting Feedback",
            State::InProgress => "In Progress",
            State::Paused => "Paused",
            State::Failed => "Failed",
            State::Completed => "Completed",
        }
    }
}

/// Automation mode enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AutomationMode {
    AutomationModeUnspecified,
    AutoCreatePr,
}

/// Source context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceContext {
    /// Required. Source name
    pub source: String,

    /// Optional. GitHub repo context
    #[serde(rename = "githubRepoContext", skip_serializing_if = "Option::is_none")]
    pub github_repo_context: Option<GitHubRepoContext>,
}

/// GitHub repository context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepoContext {
    /// Required. Starting branch name
    #[serde(rename = "startingBranch")]
    pub starting_branch: String,
}

/// Session output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOutput {
    /// Pull request output
    #[serde(rename = "pullRequest", skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
}

/// Pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    /// PR URL
    pub url: String,

    /// PR title
    pub title: String,

    /// PR description
    pub description: String,
}

/// Create session request
#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionRequest {
    pub prompt: String,
    #[serde(rename = "sourceContext")]
    pub source_context: SourceContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "requirePlanApproval",
        skip_serializing_if = "Option::is_none"
    )]
    pub require_plan_approval: Option<bool>,
    #[serde(rename = "automationMode", skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<AutomationMode>,
}

/// Send message request
#[derive(Debug, Clone, Serialize)]
pub struct SendMessageRequest {
    pub prompt: String,
}

/// List sessions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    #[serde(default)]
    pub sessions: Vec<Session>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
