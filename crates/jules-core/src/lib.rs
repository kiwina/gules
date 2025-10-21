//! # Gules Core
//!
//! Shared utilities for Gules CLI and MCP server.
//!
//! This crate contains common functionality used by both the CLI and MCP
//! server implementations, including configuration management and display
//! utilities. It's not published to crates.io as it's internal to the
//! Gules ecosystem.

pub mod activity_cache;
pub mod config;
pub mod display;

// Re-export commonly used types
pub use activity_cache::*;
pub use config::*;
pub use display::*;
