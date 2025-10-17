//! MCP tool implementations.
//!
//! This module contains the MCP tool handlers that expose Jules API
//! functionality to AI assistants in a pure 1:1 mapping.
//!
//! For extended features (watch_session, issue_status), use the gules crate
//! with the "extended-mcp" feature flag.

use jules_rs::types::*;
use rmcp::model::*;
use rmcp::ErrorData as McpError;
use schemars::JsonSchema;

use crate::server::AppState;

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct CreateSessionArgs {
    /// The prompt describing what Jules should do
    pub prompt: String,
    /// Source context (e.g., "sources/github/owner/repo")
    pub source: String,
    /// Optional session title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Starting branch for GitHub repos (default: main)
    #[serde(default = "default_branch")]
    pub branch: String,
    /// Automation mode (optional): If set to "AUTO_CREATE_PR", automatically creates a PR when complete.
    /// If omitted, no PR will be automatically created (manual mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<String>,
}

fn default_branch() -> String {
    "main".to_string()
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct GetSessionArgs {
    /// Session ID to retrieve
    pub session_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ListSessionsArgs {
    /// Page size (default: 10)
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    /// Page token for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

fn default_page_size() -> u32 {
    10
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct SendMessageArgs {
    /// Session ID
    pub session_id: String,
    /// Message to send
    pub message: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ApprovePlanArgs {
    /// Session ID
    pub session_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ListSourcesArgs {
    /// Filter sources (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Page size (default: 30)
    #[serde(default = "default_sources_page_size")]
    pub page_size: u32,
    /// Page token for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

fn default_sources_page_size() -> u32 {
    30
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct GetSourceArgs {
    /// Source ID
    pub source_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ListActivitiesArgs {
    /// Session ID
    pub session_id: String,
    /// Page size (default: 30)
    #[serde(default = "default_activities_page_size")]
    pub page_size: u32,
    /// Page token for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

fn default_activities_page_size() -> u32 {
    30
}

#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct GetActivityArgs {
    /// Session ID
    pub session_id: String,
    /// Activity ID
    pub activity_id: String,
}

/// Handler for create_session tool
pub async fn handle_create_session(
    state: &AppState,
    args: CreateSessionArgs,
) -> Result<CallToolResult, McpError> {
    let title = args.title.unwrap_or_else(|| {
        if args.prompt.len() > 50 {
            format!("{}...", &args.prompt[..47])
        } else {
            args.prompt.clone()
        }
    });

    let automation_mode = args.automation_mode.map(|mode| {
        if mode == "AUTO_CREATE_PR" {
            AutomationMode::AutoCreatePr
        } else {
            AutomationMode::AutomationModeUnspecified
        }
    });

    let request = CreateSessionRequest {
        prompt: args.prompt.clone(),
        source_context: SourceContext {
            source: args.source,
            github_repo_context: Some(GitHubRepoContext {
                starting_branch: args.branch,
            }),
        },
        title: Some(title),
        require_plan_approval: None,
        automation_mode,
    };

    let client = state.client.lock().await;

    // Use SDK method instead of .post()
    let session = client
        .create_session(request)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let session_id = session.name.clone();
    let session_url = session.url.clone().unwrap_or_default();
    let pr_url = session
        .outputs
        .iter()
        .find_map(|output| output.pull_request.as_ref())
        .map(|pr| pr.url.clone())
        .unwrap_or_default();

    Ok(CallToolResult::success(vec![
        Content::text(format!(
            "Session created successfully!\n\nSession ID: {}\nPrompt: {}\nURL: {}\nPR: {}",
            session_id, args.prompt, session_url, pr_url
        )),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&session).unwrap(),
            format!("gules://session/{}", session_id),
        )),
    ]))
}

/// Handler for get_session tool
pub async fn handle_get_session(
    state: &AppState,
    args: GetSessionArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method
    let session = client
        .get_session(&args.session_id)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let state_val = session.state.map(|s| s.display_name()).unwrap_or("Unknown");
    let title = session
        .title
        .clone()
        .unwrap_or_else(|| "No title".to_string());
    let url = session.url.clone().unwrap_or_default();
    let pr_url = session
        .outputs
        .iter()
        .find_map(|output| output.pull_request.as_ref())
        .map(|pr| pr.url.clone())
        .unwrap_or_default();

    let mut summary = format!(
        "Session: {}\nState: {}\nTitle: {}\nURL: {}",
        args.session_id, state_val, title, url
    );

    if !pr_url.is_empty() {
        summary.push_str(&format!("\nPR: {}", pr_url));
    }

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&session).unwrap(),
            format!("gules://session/{}", args.session_id),
        )),
    ]))
}

/// Handler for list_sessions tool
pub async fn handle_list_sessions(
    state: &AppState,
    args: ListSessionsArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method with all parameters
    let response = client
        .list_sessions(Some(args.page_size), args.page_token.as_deref())
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let sessions_count = response.sessions.len();

    let summary = if sessions_count == 0 {
        "No sessions found".to_string()
    } else {
        format!("Found {} session(s)", sessions_count)
    };

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&response).unwrap(),
            "gules://sessions".to_string(),
        )),
    ]))
}

/// Handler for send_message tool
pub async fn handle_send_message(
    state: &AppState,
    args: SendMessageArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method
    client
        .send_message(&args.session_id, &args.message)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    Ok(CallToolResult::success(vec![Content::text(format!(
        "Message sent successfully to session: {}\n\nUse get_session to see the updated session details.",
        args.session_id
    ))]))
}

/// Handler for approve_plan tool
pub async fn handle_approve_plan(
    state: &AppState,
    args: ApprovePlanArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method
    client
        .approve_plan(&args.session_id)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    Ok(CallToolResult::success(vec![Content::text(format!(
        "Plan approved successfully for session: {}\n\nThe session will now execute the approved plan.\nUse get_session to monitor progress.",
        args.session_id
    ))]))
}

/// Handler for list_sources tool
pub async fn handle_list_sources(
    state: &AppState,
    args: ListSourcesArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method with all parameters
    let response = client
        .list_sources(
            args.filter.as_deref(),
            Some(args.page_size),
            args.page_token.as_deref(),
        )
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let sources_count = response.sources.len();
    let next_token = response.next_page_token.clone();

    let mut summary = if sources_count == 0 {
        "No sources found".to_string()
    } else {
        format!("Found {} source(s)", sources_count)
    };

    if let Some(token) = &next_token {
        summary.push_str(&format!("\nNext page token: {}", token));
    }

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&response).unwrap(),
            "gules://sources".to_string(),
        )),
    ]))
}

/// Handler for get_source tool
pub async fn handle_get_source(
    state: &AppState,
    args: GetSourceArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method
    let source = client
        .get_source(&args.source_id)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let summary = format!("Source: {}\nID: {}", source.name, source.id);

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&source).unwrap(),
            format!("gules://source/{}", args.source_id),
        )),
    ]))
}

/// Handler for list_activities tool
pub async fn handle_list_activities(
    state: &AppState,
    args: ListActivitiesArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method with all parameters
    let response = client
        .list_activities(
            &args.session_id,
            Some(args.page_size),
            args.page_token.as_deref(),
        )
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let activities_count = response.activities.len();
    let next_token = response.next_page_token.clone();

    let mut summary = if activities_count == 0 {
        format!("No activities found for session: {}", args.session_id)
    } else {
        format!(
            "Found {} activity(ies) for session: {}",
            activities_count, args.session_id
        )
    };

    if let Some(token) = &next_token {
        summary.push_str(&format!("\nNext page token: {}", token));
    }

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&response).unwrap(),
            format!("gules://session/{}/activities", args.session_id),
        )),
    ]))
}

/// Handler for get_activity tool
pub async fn handle_get_activity(
    state: &AppState,
    args: GetActivityArgs,
) -> Result<CallToolResult, McpError> {
    let client = state.client.lock().await;

    // Use SDK method
    let activity = client
        .get_activity(&args.session_id, &args.activity_id)
        .await
        .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

    let summary = format!(
        "Activity: {}\nType: {}\nOriginator: {}",
        activity.id,
        activity.activity_type(),
        activity.originator
    );

    Ok(CallToolResult::success(vec![
        Content::text(summary),
        Content::resource(ResourceContents::text(
            serde_json::to_string_pretty(&activity).unwrap(),
            format!(
                "gules://session/{}/activity/{}",
                args.session_id, args.activity_id
            ),
        )),
    ]))
}
