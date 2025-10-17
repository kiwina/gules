//! # Jules API SDK
//!
//! A pure Rust SDK for Google's Jules AI coding agent API.
//!
//! This crate provides a type-safe, async-first client for interacting with
//! the Jules API. It's designed to be minimal and focused, with no CLI
//! dependencies.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use jules_rs::JulesClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = JulesClient::new("your-api-key");
//!     
//!     // List sessions (returns Response with sessions field)
//!     let response = client.list_sessions(Some(30), None).await?;
//!     println!("Found {} sessions", response.sessions.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod types;

// Re-export commonly used types
pub use client::{JulesClient, JulesConfig};
pub use types::*;
