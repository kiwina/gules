//! # Jules MCP Server Implementation
//!
//! Pure SDK MCP server implementation for Jules API.
//!
//! This module contains the MCP server with a 1:1 mapping to the Jules API,
//! exposing 9 core SDK tools without any extended features.
//!
//! For extended features (watch_session, issue_status), use the gules crate
//! with the "extended-mcp" feature flag.

use jules_core::config::load_config;
use jules_rs::client::JulesClient;
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
use tracing::{error, info};

use crate::tools::*;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<JulesClient>>,
}

#[derive(Clone)]
pub struct GulesServer {
    state: AppState,
    tool_router: ToolRouter<GulesServer>,
}

#[tool_router]
impl GulesServer {
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

    #[tool(description = "List available sources/repositories")]
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

    #[tool(description = "List activities in a Jules session")]
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
}

#[tool_handler]
impl ServerHandler for GulesServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "gules".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                title: Some("Gules MCP Server".to_string()),
                website_url: None,
            },
            instructions: Some(
                "Gules MCP Server - Interact with Google's Jules AI coding agent.\n\n\
                 Available SDK tools (9 pure 1:1 mappings):\n\
                 - create_session: Create a new Jules coding session\n\
                 - get_session: Get details of a session\n\
                 - list_sessions: List all sessions\n\
                 - send_message: Send a message to a session\n\
                 - approve_plan: Approve a plan in a session\n\
                 - list_sources: List available sources\n\
                 - get_source: Get details of a source\n\
                 - list_activities: List activities in a session\n\
                 - get_activity: Get details of an activity\n\n\
                 Configure API key via JULES_API_KEY environment variable or ~/.config/jules/config.toml\n\n\
                 For extended features (watch_session, issue_status), use gules with --mcp and extended-mcp feature."
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

/// Start the MCP server (SDK tools only)
pub async fn start_mcp_server() -> anyhow::Result<()> {
    info!("Starting Jules MCP server (SDK tools only)");

    // Load configuration
    let config = load_config().map_err(|e| {
        error!("Failed to load config: {}", e);
        e
    })?;

    // Create Jules API client
    let client = JulesClient::new(config.api_key.unwrap_or_default());

    // Create and run the server
    let server = GulesServer::new(client);
    if let Err(e) = server.serve_stdio().await {
        error!("MCP server error: {}", e);
        return Err(anyhow::anyhow!("MCP server error: {}", e));
    }

    Ok(())
}
