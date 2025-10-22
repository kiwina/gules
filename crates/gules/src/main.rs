//! # Gules - Extended
//!
//! Extended CLI and MCP server for Google's Jules AI coding agent.
//!
//! This crate provides the full Gules experience with GitHub integrations,
//! advanced monitoring, and optional MCP server support.
//!
//! ## Feature Flags
//!
//! - `mcp`: Enable basic MCP server with SDK tools only (9 tools)
//! - `extended-mcp`: Enable extended MCP server with SDK + extended tools (11 tools)

use clap::{Parser, Subcommand};
use jules_cli::commands::*;

mod commands;
mod extended_commands;

#[cfg(feature = "mcp")]
mod mcp;

/// Output format for CLI commands
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Table,
    Full,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "table" => Ok(Self::Table),
            "full" => Ok(Self::Full),
            _ => anyhow::bail!("Unknown output format: {}. Valid options: json, table, full", s),
        }
    }
}

#[derive(Parser)]
#[command(name = "gules")]
#[command(version)]
#[command(about = "Extended CLI and MCP server for Google's Jules AI coding agent")]
#[command(
    long_about = "Full-featured Jules AI companion with GitHub integrations, advanced monitoring, and MCP server support."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run as MCP server instead of CLI
    #[cfg(feature = "mcp")]
    #[arg(long)]
    mcp: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all sessions
    Sessions {
        /// Filter by state: active, completed, failed, or paused
        #[arg(long, value_name = "STATE")]
        state: Option<String>,
        /// Search text in session titles or prompts
        #[arg(long, value_name = "TEXT")]
        search: Option<String>,
        /// Maximum number of sessions (1-100, default: 50)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Get detailed information about a specific session
    Session {
        /// Session ID (long numeric string)
        #[arg(value_name = "SESSION_ID")]
        id: String,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// List only active sessions (convenience filter)
    Active {
        /// Search text in titles and prompts
        #[arg(long, value_name = "TEXT")]
        search: Option<String>,
        /// Maximum number of results (1-100)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// List only completed sessions (convenience filter)
    Completed {
        /// Search text in titles and prompts
        #[arg(long, value_name = "TEXT")]
        search: Option<String>,
        /// Maximum number of results (1-100)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// List only failed sessions (convenience filter)
    Failed {
        /// Search text in titles and prompts
        #[arg(long, value_name = "TEXT")]
        search: Option<String>,
        /// Maximum number of results (1-100)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Create a new Jules AI coding session
    Create {
        /// Task description for Jules (be specific!)
        #[arg(value_name = "PROMPT")]
        prompt: String,
        /// Source repository (format: sources/github/owner/repo)
        #[arg(short, long, value_name = "SOURCE")]
        source: String,
        /// Optional session title (shown in UI)
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,
        /// Starting branch for GitHub repos (default: main)
        #[arg(long, value_name = "BRANCH")]
        branch: Option<String>,
        /// Require plan approval before execution (default: false)
        #[arg(long, default_value = "false")]
        require_approval: bool,
        /// Automation mode: AUTO_CREATE_PR or MANUAL (default: AUTO_CREATE_PR)
        #[arg(long, default_value = "AUTO_CREATE_PR", value_name = "MODE")]
        automation_mode: String,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// List available code sources/repositories
    Sources {
        /// AIP-160 filter (e.g., "name=sources/github/owner/repo")
        #[arg(long, value_name = "FILTER")]
        filter: Option<String>,
        /// Maximum number of results (1-100)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Get detailed information about a specific source
    Source {
        /// Source ID (format: sources/github/owner/repo)
        #[arg(value_name = "SOURCE_ID")]
        id: String,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// List all activities in a session
    Activities {
        /// Session ID to list activities for
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
        /// Maximum number of activities (1-100)
        #[arg(long, default_value = "50", value_name = "NUM")]
        limit: u32,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Get detailed information about a specific activity
    Activity {
        /// Session ID containing the activity
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
        /// Activity ID (long numeric string)
        #[arg(value_name = "ACTIVITY_ID")]
        activity_id: String,
        /// Output format: json, table, full (default: json)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Send a message to an active Jules session
    SendMessage {
        /// Session ID to send message to
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
        /// Message text (be clear and specific)
        #[arg(value_name = "MESSAGE")]
        message: String,
    },
    /// Approve the execution plan for a session
    ApprovePlan {
        /// Session ID with pending plan approval
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Check Jules sessions linked to a GitHub issue (requires gh CLI)
    IssueStatus {
        /// GitHub issue number
        #[arg(value_name = "ISSUE_NUM")]
        issue: u32,
        /// GitHub repository owner/organization
        #[arg(short, long, value_name = "OWNER")]
        owner: String,
        /// GitHub repository name
        #[arg(short, long, value_name = "REPO")]
        repo: String,
    },
    /// Find the GitHub PR created by a Jules session (requires gh CLI)
    PrStatus {
        /// Session ID that created the PR
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
    },
    /// Continuously monitor session until completion
    Watch {
        /// Session ID
        session_id: String,
        /// Poll interval in seconds
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
    /// Continuously monitor all sessions
    Monitor {
        /// Poll interval in seconds
        #[arg(short, long, default_value = "30")]
        interval: u64,
    },
    /// Filter and search session activities with caching
    FilterActivities {
        /// Session ID to filter activities for
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
        /// Get only the last N activities
        #[arg(long, value_name = "N")]
        last: Option<usize>,
        /// Filter by activity type (comma-separated)
        /// Types: agent-message, user-message, plan, progress, completed, failed, error
        #[arg(long, value_name = "TYPES", value_delimiter = ',')]
        r#type: Vec<String>,
        /// Filter activities with bash output (test errors, command outputs)
        #[arg(long)]
        has_bash_output: bool,
        /// Disable cache and fetch fresh from API
        #[arg(long)]
        no_cache: bool,
        /// Output format: json (default, machine-readable), table (human-readable), full (detailed), content-only (text only)
        #[arg(long, default_value = "json", value_name = "FORMAT")]
        format: String,
    },
    /// Manage activity cache
    Cache {
        #[command(subcommand)]
        action: CacheCommands,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Initialize configuration
    Init,
    /// Set a configuration value
    Set {
        /// Key to set
        key: String,
        /// Value to set
        value: String,
    },
}

#[derive(Subcommand)]
enum CacheCommands {
    /// Show cache statistics
    Stats,
    /// Clear all cached activities
    Clear,
    /// Delete cache for a specific session
    Delete {
        /// Session ID to delete cache for
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Check if running as MCP server
    #[cfg(feature = "mcp")]
    if cli.mcp {
        return run_mcp_server().await;
    }

    // CLI mode
    match cli.command {
        Some(Commands::Sessions {
            state,
            search,
            limit,
            format,
        }) => {
            extended_commands::handle_sessions_formatted(state, search, limit, &format).await?;
        }
        Some(Commands::Session { id, format }) => {
            extended_commands::handle_session_formatted(&id, &format).await?;
        }
        Some(Commands::Active { search, limit, format }) => {
            extended_commands::handle_active_formatted(search, limit, &format).await?;
        }
        Some(Commands::Completed { search, limit, format }) => {
            extended_commands::handle_completed_formatted(search, limit, &format).await?;
        }
        Some(Commands::Failed { search, limit, format }) => {
            extended_commands::handle_failed_formatted(search, limit, &format).await?;
        }
        Some(Commands::Create {
            prompt,
            source,
            title,
            branch,
            require_approval,
            automation_mode,
            format,
        }) => {
            extended_commands::handle_create_formatted(
                prompt,
                source,
                title,
                branch,
                require_approval,
                &automation_mode,
                &format,
            ).await?;
        }
        Some(Commands::Sources { filter, limit, format }) => {
            extended_commands::handle_sources_formatted(filter, limit, &format).await?;
        }
        Some(Commands::Source { id, format }) => {
            extended_commands::handle_source_formatted(&id, &format).await?;
        }
        Some(Commands::Activities { session_id, limit, format }) => {
            extended_commands::handle_activities_formatted(&session_id, limit, &format).await?;
        }
        Some(Commands::Activity {
            session_id,
            activity_id,
            format,
        }) => {
            extended_commands::handle_activity_formatted(&session_id, &activity_id, &format).await?;
        }
        Some(Commands::SendMessage {
            session_id,
            message,
        }) => {
            let args = SendMessageArgs {
                session_id,
                message,
            };
            handle_send_message(args).await?;
        }
        Some(Commands::ApprovePlan { session_id }) => {
            let args = ApprovePlanArgs { session_id };
            handle_approve_plan(args).await?;
        }
        Some(Commands::Config { action }) => match action {
            ConfigCommands::Show => {
                let args = ConfigShowArgs;
                handle_config_show(args).await?;
            }
            ConfigCommands::Init => {
                let args = ConfigInitArgs;
                handle_config_init(args).await?;
            }
            ConfigCommands::Set { key, value } => {
                let args = ConfigSetArgs { key, value };
                handle_config_set(args).await?;
            }
        },
        Some(Commands::IssueStatus { issue, owner, repo }) => {
            extended_commands::handle_issue_status(issue, &owner, &repo).await?;
        }
        Some(Commands::PrStatus { session_id }) => {
            extended_commands::handle_pr_status(&session_id).await?;
        }
        Some(Commands::Watch {
            session_id,
            interval,
        }) => {
            extended_commands::handle_watch(&session_id, interval).await?;
        }
        Some(Commands::Monitor { interval }) => {
            extended_commands::handle_monitor(interval).await?;
        }
        Some(Commands::FilterActivities {
            session_id,
            last,
            r#type,
            has_bash_output,
            no_cache,
            format,
        }) => {
            use commands::filter_activities::*;

            // Parse type filters
            let type_filters: Result<Vec<ActivityTypeFilter>, _> = r#type
                .iter()
                .map(|s| ActivityTypeFilter::from_str(s))
                .collect();
            let type_filters = type_filters?;

            // Parse output format
            let output_format = OutputFormat::from_str(&format)?;

            filter_activities(
                &session_id,
                last,
                type_filters,
                has_bash_output,
                no_cache,
                output_format,
            )
            .await?;
        }
        Some(Commands::Cache { action }) => match action {
            CacheCommands::Stats => {
                commands::handle_cache_stats().await?;
            }
            CacheCommands::Clear => {
                commands::handle_cache_clear().await?;
            }
            CacheCommands::Delete { session_id } => {
                commands::handle_cache_delete(&session_id).await?;
            }
        },
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}

#[cfg(feature = "mcp")]
async fn run_mcp_server() -> anyhow::Result<()> {
    #[cfg(feature = "extended-mcp")]
    {
        // Extended MCP server with SDK + extended tools (11 tools)
        mcp::start_extended_mcp_server().await
    }

    #[cfg(not(feature = "extended-mcp"))]
    {
        // Basic MCP server with SDK tools only (9 tools)
        jules_mcp::start_mcp_server().await
    }
}
