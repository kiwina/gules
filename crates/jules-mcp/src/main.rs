//! # Gules MCP Server
//!
//! MCP server for Google's Jules AI coding agent.
//!
//! This crate provides an MCP (Model Context Protocol) server that allows
//! AI assistants like Claude Desktop to interact with Jules AI.

use tracing::{error, info};

mod server;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    info!("Starting Gules MCP server");

    if let Err(e) = server::start_mcp_server().await {
        error!("MCP server error: {}", e);
        return Err(e);
    }

    Ok(())
}
