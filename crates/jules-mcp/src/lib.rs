//! # Jules MCP Server
//!
//! Pure SDK MCP server implementation for Google's Jules AI coding agent.
//!
//! This crate provides MCP (Model Context Protocol) tools that expose
//! the Jules API in a 1:1 mapping, with no extended features.
//!
//! For extended features (watch_session, issue_status, etc.), use the
//! gules crate with the "extended-mcp" feature flag.

pub mod server;
pub mod tools;

pub use server::start_mcp_server;
