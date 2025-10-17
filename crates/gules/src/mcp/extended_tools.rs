//! Extended MCP tool implementations for gules.
//!
//! These tools provide additional functionality beyond the pure SDK,
//! including session monitoring and GitHub integration.

use jules_mcp::server::AppState;
use jules_rs::types::State;
use rmcp::model::*;
use rmcp::ErrorData as McpError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct WatchSessionArgs {
    /// Session ID to watch
    pub session_id: String,
    /// Check interval in seconds (default: 10)
    #[serde(default = "default_interval")]
    pub interval: u64,
    /// Maximum wait time in seconds (default: 600 = 10 minutes)
    #[serde(default = "default_max_wait")]
    pub max_wait: u64,
}

fn default_interval() -> u64 {
    10
}

fn default_max_wait() -> u64 {
    600
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct IssueStatusArgs {
    /// GitHub issue number
    pub issue: u32,
    /// Repository owner
    pub owner: String,
    /// Repository name
    pub repo: String,
}

/// Handler for watch_session tool (extended feature)
pub async fn handle_watch_session(
    state: &AppState,
    args: WatchSessionArgs,
) -> Result<CallToolResult, McpError> {
    let start_time = std::time::Instant::now();
    let max_duration = std::time::Duration::from_secs(args.max_wait);

    let mut last_state = String::new();

    loop {
        if start_time.elapsed() > max_duration {
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Timeout: Session did not complete within {} seconds",
                args.max_wait
            ))]));
        }

        let client_guard = state.client.lock().await;

        let session = client_guard
            .get_session(&args.session_id)
            .await
            .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

        if let Some(state_val) = session.state {
            let state_str = state_val.display_name().to_string();
            if state_str != last_state {
                last_state = state_str;
            }

            if state_val == State::Completed || state_val == State::Failed {
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

                let mut result = format!(
                    "Session {} - Final state: {}\nTitle: {}\nURL: {}",
                    args.session_id,
                    state_val.display_name(),
                    title,
                    url
                );

                if !pr_url.is_empty() {
                    result.push_str(&format!("\nPR created: {}", pr_url));
                }

                return Ok(CallToolResult::success(vec![
                    Content::text(result),
                    Content::resource(ResourceContents::text(
                        serde_json::to_string_pretty(&session).unwrap(),
                        format!("gules://session/{}", args.session_id),
                    )),
                ]));
            }
        }

        drop(client_guard);
        tokio::time::sleep(tokio::time::Duration::from_secs(args.interval)).await;
    }
}

/// Handler for issue_status tool (extended feature)
pub async fn handle_issue_status(
    _state: &AppState,
    args: IssueStatusArgs,
) -> Result<CallToolResult, McpError> {
    // This tool requires gh CLI integration
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Checking issue #{} in {}/{} for Jules sessions...\n\n\
         Note: The issue-status command requires GitHub CLI (gh) to be installed.\n\
         \n\
         To use this feature:\n\
         1. Install gh CLI: https://cli.github.com\n\
         2. Run: gules issue-status {} --owner {} --repo {}\n\
         \n\
         The CLI version provides full GitHub integration including:\n\
         - Reading issue comments for Jules session IDs\n\
         - Fetching session details from Jules API\n\
         - Displaying PR information if available",
        args.issue, args.owner, args.repo, args.issue, args.owner, args.repo
    ))]))
}
