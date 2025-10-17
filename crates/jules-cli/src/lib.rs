//! # Gules CLI Library
//!
//! Library interface for the Gules CLI commands.
//!
//! This library provides the command handlers that can be used by other
//! crates, such as the extended CLI that adds GitHub integrations.

pub mod commands;

// Re-export command types and handlers
pub use commands::*;
