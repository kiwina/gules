//! MCP server module for gules.
//!
//! This module provides MCP server implementations with different feature sets:
//! - Basic MCP (feature "mcp"): Pure SDK tools only (9 tools) - uses jules-mcp directly
//! - Extended MCP (feature "extended-mcp"): SDK tools + extended features (11 tools)

#[cfg(feature = "extended-mcp")]
mod extended_server;
#[cfg(feature = "extended-mcp")]
mod extended_tools;

#[cfg(feature = "extended-mcp")]
pub use extended_server::start_extended_mcp_server;
