//! Extended CLI command implementations.
//!
//! This module contains extended command implementations that require
//! external tools like the GitHub CLI or provide advanced monitoring
//! features not available in the basic CLI.

pub mod issue_status;
pub mod pr_status;
pub mod watch;
pub mod monitor;

// Re-export extended command handlers
pub use issue_status::*;
pub use pr_status::*;
pub use watch::*;
pub use monitor::*;