//! Extended MCP server implementation for gules.
//!
//! This server includes both SDK tools (from jules-mcp) and extended tools
//! (watch_session, issue_status) for enhanced functionality.
//!
//! NOTE: Due to rmcp framework limitations, we must redeclare all SDK tools
//! here to add extended tools. The handlers are still delegated to jules-mcp
//! to avoid logic duplication. This is architectural debt that can be resolved
//! when rmcp supports tool composition/extension.

use anyhow::Result;
use jules_rs::JulesClient;
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::io::stdio,
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::extended_tools::{IssueStatusArgs, WatchSessionArgs};

// Re-use AppState from jules-mcp
use jules_mcp::server::AppState;

// Delegate to SDK tool handlers from jules-mcp (no logic duplication)
use jules_mcp::tools::{
    handle_approve_plan, handle_create_session, handle_get_activity, handle_get_session,
    handle_get_source, handle_list_activities, handle_list_sessions, handle_list_sources,
    handle_send_message, ApprovePlanArgs, CreateSessionArgs, GetActivityArgs, GetSessionArgs,
    GetSourceArgs, ListActivitiesArgs, ListSessionsArgs, ListSourcesArgs, SendMessageArgs,
};

#[derive(Clone)]
pub struct GalesExtendedServer {
    state: AppState,
    tool_router: ToolRouter<GalesExtendedServer>,
}

#[tool_router]
impl GalesExtendedServer {
    pub fn new(client: JulesClient) -> Self {
        let state = AppState {
            client: Arc::new(Mutex::new(client)),
        };
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }

    pub async fn serve_stdio(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    // === SDK Tools (9 total - delegated to jules-mcp handlers) ===
    // NOTE: Tool registration required by rmcp, but handlers reuse jules-mcp logic

    #[tool(
        description = "Create a new Jules AI coding session that will automatically create a PR"
    )]
    async fn create_session(
        &self,
        Parameters(args): Parameters<CreateSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_create_session(&self.state, args).await
    }

    #[tool(description = "Get details of a specific Jules session")]
    async fn get_session(
        &self,
        Parameters(args): Parameters<GetSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_get_session(&self.state, args).await
    }

    #[tool(description = "List Jules sessions")]
    async fn list_sessions(
        &self,
        Parameters(args): Parameters<ListSessionsArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_list_sessions(&self.state, args).await
    }

    #[tool(description = "Send a message to a Jules session")]
    async fn send_message(
        &self,
        Parameters(args): Parameters<SendMessageArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_send_message(&self.state, args).await
    }

    #[tool(description = "Approve a plan in a Jules session")]
    async fn approve_plan(
        &self,
        Parameters(args): Parameters<ApprovePlanArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_approve_plan(&self.state, args).await
    }

    #[tool(description = "List available sources (repositories)")]
    async fn list_sources(
        &self,
        Parameters(args): Parameters<ListSourcesArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_list_sources(&self.state, args).await
    }

    #[tool(description = "Get details of a specific source")]
    async fn get_source(
        &self,
        Parameters(args): Parameters<GetSourceArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_get_source(&self.state, args).await
    }

    #[tool(description = "List activities in a session")]
    async fn list_activities(
        &self,
        Parameters(args): Parameters<ListActivitiesArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_list_activities(&self.state, args).await
    }

    #[tool(description = "Get details of a specific activity")]
    async fn get_activity(
        &self,
        Parameters(args): Parameters<GetActivityArgs>,
    ) -> Result<CallToolResult, McpError> {
        handle_get_activity(&self.state, args).await
    }

    // === Extended Tools (2 total) ===

    #[tool(description = "Watch a Jules session until it completes or times out")]
    async fn watch_session(
        &self,
        Parameters(args): Parameters<WatchSessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        // Inline extended logic
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

            let client = self.state.client.lock().await;
            let session = client
                .get_session(&args.session_id)
                .await
                .map_err(|e| McpError::internal_error(format!("API error: {}", e), None))?;

            if let Some(state_val) = session.state {
                let state_str = state_val.display_name().to_string();
                if state_str != last_state {
                    last_state = state_str;
                }

                use jules_rs::types::State;
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

            drop(client);
            tokio::time::sleep(tokio::time::Duration::from_secs(args.interval)).await;
        }
    }

    #[tool(description = "Check Jules sessions linked to a GitHub issue")]
    async fn issue_status(
        &self,
        Parameters(args): Parameters<IssueStatusArgs>,
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
}

#[tool_handler]
impl ServerHandler for GalesExtendedServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "gules-extended".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                title: Some("Gules Extended MCP Server".to_string()),
                website_url: None,
            },
            instructions: Some(
                "Gules Extended MCP Server - Full-featured MCP for Google's Jules AI coding agent.\n\n\
                 SDK Tools (9 tools):\n\
                 - create_session: Create a new coding session\n\
                 - get_session: Get details of a session\n\
                 - list_sessions: List all sessions\n\
                 - send_message: Send a message to a session\n\
                 - approve_plan: Approve a plan in a session\n\
                 - list_sources: List available sources\n\
                 - get_source: Get details of a source\n\
                 - list_activities: List activities in a session\n\
                 - get_activity: Get details of an activity\n\n\
                 Extended Tools (2 tools):\n\
                 - watch_session: Monitor a session until completion (polling)\n\
                 - issue_status: Check GitHub issues for Jules sessions\n\n\
                 Configure API key via JULES_API_KEY environment variable or ~/.config/jules/config.toml"
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}

/// Start the extended Gules MCP server with SDK + extended tools
pub async fn start_extended_mcp_server() -> Result<()> {
    // Load configuration
    let config = jules_core::config::load_config()?;
    let api_key = config.api_key.ok_or_else(|| {
        anyhow::anyhow!(
            "API key not found. Please run 'gules config init' or set JULES_API_KEY environment variable"
        )
    })?;

    // Create client
    let client = JulesClient::new(api_key);

    // Create and run the server
    let server = GalesExtendedServer::new(client);
    if let Err(e) = server.serve_stdio().await {
        return Err(anyhow::anyhow!("MCP server error: {}", e));
    }

    Ok(())
}
