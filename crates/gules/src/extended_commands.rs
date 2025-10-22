//! # Extended Commands
//!
//! Commands that require external dependencies or advanced features
//! not available in the basic gules-cli crate.

use anyhow::{Context, Result};
use chrono::Local;
use jules_core::config::load_config;
use jules_rs::JulesClient;
use std::process::Command;
use tokio::time::{sleep, Duration};

/// Output format for CLI commands
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Table,
    Full,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "table" => Ok(Self::Table),
            "full" => Ok(Self::Full),
            _ => anyhow::bail!("Unknown output format: {}. Valid options: json, table, full", s),
        }
    }
}

/// Handle issue-status command (requires gh CLI)
pub async fn handle_issue_status(issue: u32, owner: &str, repo: &str) -> Result<()> {
    // Check if gh CLI is available
    if !is_gh_cli_available() {
        anyhow::bail!(
            "GitHub CLI (gh) is required for the issue-status command.\n\
             Install from: https://cli.github.com\n\
             \n\
             Installation options:\n\
             - Linux (apt):   sudo apt install gh\n\
             - Linux (dnf):   sudo dnf install gh\n\
             - macOS (brew):  brew install gh\n\
             - Windows:       winget install --id GitHub.cli\n\
             - Or download from: https://github.com/cli/cli/releases"
        );
    }

    // Load API key
    let config = load_config()?;
    let api_key = config
        .api_key
        .context("API key not configured. Run 'gules config init'")?;
    let client = JulesClient::new(&api_key);

    // Get issue comments via gh CLI
    let comments = get_issue_comments_via_gh(owner, repo, issue)?;

    // Parse comments for Jules session IDs
    let session_ids = extract_jules_session_ids(&comments);

    if session_ids.is_empty() {
        println!(
            "No Jules sessions found in {}/{}#{} comments",
            owner, repo, issue
        );
        return Ok(());
    }

    println!(
        "Found {} Jules session(s) for {}/{}#{}:\n",
        session_ids.len(),
        owner,
        repo,
        issue
    );

    // Fetch and display session details
    for session_id in session_ids {
        match client.get_session(&session_id).await {
            Ok(session) => {
                println!("Session: {}", session.id);
                if let Some(title) = &session.title {
                    println!("  Title: {}", title);
                }
                println!("  State: {:?}", session.state);
                if let Some(create_time) = &session.create_time {
                    println!("  Created: {}", create_time);
                }

                // Show PR if available
                if !session.outputs.is_empty() {
                    for output in &session.outputs {
                        if let Some(pr) = &output.pull_request {
                            println!("  PR URL: {}", pr.url);
                            println!("  PR Title: {}", pr.title);
                        }
                    }
                }
                println!();
            }
            Err(e) => {
                eprintln!("Failed to fetch session {}: {}", session_id, e);
            }
        }
    }

    Ok(())
}

/// Handle pr-status command (requires gh CLI)
pub async fn handle_pr_status(session_id: &str) -> Result<()> {
    // Load API key
    let config = load_config()?;
    let api_key = config
        .api_key
        .context("API key not configured. Run 'gules config init'")?;
    let client = JulesClient::new(&api_key);

    // Get session details
    let session = client.get_session(session_id).await?;

    // Extract PR info from session outputs
    if session.outputs.is_empty() {
        println!("No outputs found for session {}", session_id);
        return Ok(());
    }

    let mut found_pr = false;
    for output in session.outputs {
        if let Some(pr) = output.pull_request {
            found_pr = true;
            println!("PR Information for session {}:\n", session_id);
            println!("  Title: {}", pr.title);
            println!("  URL: {}", pr.url);
            println!("  Description: {}", pr.description);

            // Optionally fetch PR details via gh CLI
            if is_gh_cli_available() {
                if let Ok(pr_details) = get_pr_details_via_gh(&pr.url) {
                    println!("\nGitHub PR Details:");
                    for (key, value) in pr_details {
                        println!("  {}: {}", key, value);
                    }
                }
            } else {
                println!("\nNote: Install GitHub CLI (gh) for detailed PR status.");
                println!("  https://cli.github.com");
            }
        }
    }

    if !found_pr {
        println!("No PR found in outputs for session {}", session_id);
    }

    Ok(())
}

/// Handle watch command with real-time monitoring
pub async fn handle_watch(session_id: &str, interval: u64) -> Result<()> {
    // Load API key
    let config = load_config()?;
    let api_key = config
        .api_key
        .context("API key not configured. Run 'gules config init'")?;
    let client = JulesClient::new(&api_key);

    println!(
        "Watching session {} (polling every {}s)...",
        session_id, interval
    );
    println!("Press Ctrl+C to stop monitoring\n");

    let mut last_activity_count = 0;

    loop {
        // Get current session status
        match client.get_session(session_id).await {
            Ok(session) => {
                // Display session header
                println!("\n─── Session Status ────────────────────────────");
                if let Some(title) = &session.title {
                    println!("Title: {}", title);
                }
                println!("State: {:?}", session.state);
                if let Some(create_time) = &session.create_time {
                    println!("Created: {}", create_time);
                }

                // Check if session is in terminal state
                let is_terminal = matches!(
                    session.state,
                    Some(jules_rs::State::Completed)
                        | Some(jules_rs::State::Failed)
                        | Some(jules_rs::State::Paused)
                );

                if is_terminal {
                    println!("\n✓ Session reached terminal state: {:?}", session.state);
                    break;
                }

                // Try to fetch latest activities
                if let Ok(activities_response) =
                    client.list_activities(session_id, Some(5), None).await
                {
                    let activities = activities_response.activities;
                    if activities.len() != last_activity_count {
                        println!("\nRecent Activities:");
                        for activity in activities.iter().take(3) {
                            let desc = activity
                                .description
                                .as_deref()
                                .unwrap_or("(no description)");
                            println!("  • {} - {}", activity.id, desc);
                        }
                        last_activity_count = activities.len();
                    }
                }

                println!("Last updated: {}", Local::now().format("%H:%M:%S"));
            }
            Err(e) => {
                eprintln!("Error fetching session status: {}", e);
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }

    Ok(())
}

/// Handle monitor command for all sessions
pub async fn handle_monitor(interval: u64) -> Result<()> {
    // Load API key
    let config = load_config()?;
    let api_key = config
        .api_key
        .context("API key not configured. Run 'gules config init'")?;
    let client = JulesClient::new(&api_key);

    println!("Monitoring all sessions (polling every {}s)...", interval);
    println!("Press Ctrl+C to stop monitoring\n");

    loop {
        // Get all sessions
        match client.list_sessions(Some(100), None).await {
            Ok(response) => {
                let sessions = response.sessions;

                if sessions.is_empty() {
                    println!("No sessions found");
                } else {
                    println!(
                        "\n─── Sessions Summary ─────────────────────────── ({} sessions)",
                        sessions.len()
                    );
                    println!(
                        "{:<20} {:<25} {:<15} {:<20}",
                        "ID", "Title", "State", "Created"
                    );
                    println!("{}", "─".repeat(80));

                    for session in &sessions {
                        let title = session
                            .title
                            .as_deref()
                            .unwrap_or("(no title)")
                            .chars()
                            .take(25)
                            .collect::<String>();

                        let state_str = session
                            .state
                            .as_ref()
                            .map(|s| format!("{:?}", s))
                            .unwrap_or_else(|| "Unknown".to_string());

                        let created = session
                            .create_time
                            .as_deref()
                            .unwrap_or("N/A")
                            .chars()
                            .take(19)
                            .collect::<String>();

                        println!(
                            "{:<20} {:<25} {:<15} {:<20}",
                            session.id.chars().take(20).collect::<String>(),
                            title,
                            state_str.chars().take(15).collect::<String>(),
                            created
                        );
                    }
                }

                println!("\nLast updated: {}", Local::now().format("%H:%M:%S"));
            }
            Err(e) => {
                eprintln!("Error fetching sessions: {}", e);
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Helper Functions
// ─────────────────────────────────────────────────────────────────────────

/// Check if gh CLI is available
fn is_gh_cli_available() -> bool {
    Command::new("gh")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get issue comments via gh CLI
fn get_issue_comments_via_gh(owner: &str, repo: &str, issue: u32) -> Result<Vec<String>> {
    let output = Command::new("gh")
        .arg("issue")
        .arg("view")
        .arg(issue.to_string())
        .arg("--repo")
        .arg(format!("{}/{}", owner, repo))
        .arg("--json")
        .arg("comments")
        .output()
        .context("Failed to run gh CLI")?;

    if !output.status.success() {
        anyhow::bail!("gh CLI failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Parse JSON output
    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;

    // Extract comment bodies
    let comments = json["comments"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|c| c["body"].as_str())
        .map(|s| s.to_string())
        .collect();

    Ok(comments)
}

/// Extract Jules session IDs from comments
fn extract_jules_session_ids(comments: &[String]) -> Vec<String> {
    let mut session_ids = Vec::new();
    let patterns = [
        r"sessions/([a-zA-Z0-9_-]+)",
        r"session[:\s]+([a-zA-Z0-9_-]+)",
        r"https://console\.cloud\.google\.com/[^/]*/([a-zA-Z0-9_-]+)",
    ];

    for comment in comments {
        for pattern_str in &patterns {
            if let Ok(pattern) = regex::Regex::new(pattern_str) {
                for cap in pattern.captures_iter(comment) {
                    if let Some(session_id) = cap.get(1) {
                        let id = session_id.as_str().to_string();
                        if !session_ids.contains(&id) {
                            session_ids.push(id);
                        }
                    }
                }
            }
        }
    }

    session_ids
}

/// Get PR details via gh CLI
fn get_pr_details_via_gh(pr_url: &str) -> Result<Vec<(String, String)>> {
    // Extract owner/repo/pr-number from URL
    // Format: https://github.com/{owner}/{repo}/pull/{number}
    let parts: Vec<&str> = pr_url.split('/').collect();
    if parts.len() < 7 || parts[4] != "pull" {
        anyhow::bail!("Invalid PR URL format");
    }

    let owner = parts[3];
    let repo = parts[4];
    let pr_number = parts[6];

    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("--repo")
        .arg(format!("{}/{}", owner, repo))
        .arg("--json")
        .arg("state,title,author,createdAt,mergedAt")
        .output()
        .context("Failed to run gh PR view")?;

    if !output.status.success() {
        anyhow::bail!("gh PR view failed");
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;

    let mut details = vec![];
    if let Some(state) = json["state"].as_str() {
        details.push(("State".to_string(), state.to_string()));
    }
    if let Some(title) = json["title"].as_str() {
        details.push(("Title".to_string(), title.to_string()));
    }
    if let Some(author) = json["author"]["login"].as_str() {
        details.push(("Author".to_string(), author.to_string()));
    }
    if let Some(created) = json["createdAt"].as_str() {
        details.push(("Created".to_string(), created.to_string()));
    }
    if let Some(merged) = json["mergedAt"].as_str() {
        details.push(("Merged".to_string(), merged.to_string()));
    }

    Ok(details)
}

// ─────────────────────────────────────────────────────────────────────────
// Formatted Output Handlers
// ─────────────────────────────────────────────────────────────────────────

/// Handle sessions command with format support
pub async fn handle_sessions_formatted(
    state: Option<String>,
    search: Option<String>,
    limit: u32,
    format: &str,
) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let response = client.list_sessions(Some(limit), None).await?;
    let sessions = response.sessions;

    // Apply filters
    let filtered: Vec<_> = sessions.into_iter().filter(|session| {
        // State filter
        if let Some(ref state_filter) = state {
            if let Some(ref session_state) = session.state {
                let state_matches = match state_filter.to_lowercase().as_str() {
                    "active" => matches!(
                        session_state,
                        jules_rs::State::Queued
                            | jules_rs::State::Planning
                            | jules_rs::State::AwaitingPlanApproval
                            | jules_rs::State::AwaitingUserFeedback
                            | jules_rs::State::InProgress
                    ),
                    "completed" => matches!(session_state, jules_rs::State::Completed),
                    "failed" => matches!(session_state, jules_rs::State::Failed),
                    "paused" => matches!(session_state, jules_rs::State::Paused),
                    _ => true,
                };
                if !state_matches {
                    return false;
                }
            }
        }

        // Search filter
        if let Some(ref search_term) = search {
            let search_lower = search_term.to_lowercase();
            let title_match = session.title.as_ref().map(|t| t.to_lowercase().contains(&search_lower)).unwrap_or(false);
            let prompt_match = session.prompt.to_lowercase().contains(&search_lower);
            if !title_match && !prompt_match {
                return false;
            }
        }

        true
    }).collect();

    // Output based on format
    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&filtered)?);
        }
        OutputFormat::Table => {
            jules_core::display::display_sessions_table(&filtered);
        }
        OutputFormat::Full => {
            for session in &filtered {
                println!("{}", serde_json::to_string_pretty(&session)?);
                println!("─────────────────────────────────────────");
            }
        }
    }

    Ok(())
}

/// Handle session command with format support
pub async fn handle_session_formatted(id: &str, format: &str) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let session = client.get_session(id).await?;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json | OutputFormat::Full => {
            println!("{}", serde_json::to_string_pretty(&session)?);
        }
        OutputFormat::Table => {
            jules_core::display::display_sessions_table(&[session]);
        }
    }

    Ok(())
}

/// Handle active sessions with format support
pub async fn handle_active_formatted(search: Option<String>, limit: u32, format: &str) -> Result<()> {
    handle_sessions_formatted(Some("active".to_string()), search, limit, format).await
}

/// Handle completed sessions with format support
pub async fn handle_completed_formatted(search: Option<String>, limit: u32, format: &str) -> Result<()> {
    handle_sessions_formatted(Some("completed".to_string()), search, limit, format).await
}

/// Handle failed sessions with format support
pub async fn handle_failed_formatted(search: Option<String>, limit: u32, format: &str) -> Result<()> {
    handle_sessions_formatted(Some("failed".to_string()), search, limit, format).await
}

/// Handle create command with format support
pub async fn handle_create_formatted(
    prompt: String,
    source: String,
    title: Option<String>,
    branch: Option<String>,
    require_approval: bool,
    automation_mode: &str,
    format: &str,
) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    // Parse automation mode
    let automation = match automation_mode.to_uppercase().as_str() {
        "AUTO_CREATE_PR" => jules_rs::types::session::AutomationMode::AutoCreatePr,
        _ => jules_rs::types::session::AutomationMode::AutomationModeUnspecified,
    };

    // Build source context with optional branch
    let source_context = jules_rs::types::session::SourceContext {
        source: source.clone(),
        github_repo_context: branch.map(|b| jules_rs::types::session::GitHubRepoContext {
            starting_branch: b,
        }),
    };

    let request = jules_rs::types::session::CreateSessionRequest {
        prompt: prompt.clone(),
        title,
        source_context,
        require_plan_approval: Some(require_approval),
        automation_mode: Some(automation),
    };

    let session = client.create_session(request).await?;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json | OutputFormat::Full => {
            println!("{}", serde_json::to_string_pretty(&session)?);
        }
        OutputFormat::Table => {
            println!("✓ Session created successfully");
            jules_core::display::display_sessions_table(&[session]);
        }
    }

    Ok(())
}

/// Handle sources command with format support
pub async fn handle_sources_formatted(filter: Option<String>, limit: u32, format: &str) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let response = client.list_sources(filter.as_deref(), Some(limit), None).await?;
    let sources = response.sources;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&sources)?);
        }
        OutputFormat::Table => {
            jules_core::display::print_sources_table(&sources);
        }
        OutputFormat::Full => {
            for source in &sources {
                println!("{}", serde_json::to_string_pretty(&source)?);
                println!("─────────────────────────────────────────");
            }
        }
    }

    Ok(())
}

/// Handle source command with format support
pub async fn handle_source_formatted(id: &str, format: &str) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let source = client.get_source(id).await?;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json | OutputFormat::Full => {
            println!("{}", serde_json::to_string_pretty(&source)?);
        }
        OutputFormat::Table => {
            jules_core::display::print_sources_table(&[source]);
        }
    }

    Ok(())
}

/// Handle activities command with format support
pub async fn handle_activities_formatted(session_id: &str, limit: u32, format: &str) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let response = client.list_activities(session_id, Some(limit), None).await?;
    let activities = response.activities;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&activities)?);
        }
        OutputFormat::Table => {
            let refs: Vec<_> = activities.iter().collect();
            jules_core::display::print_activities_table(&refs);
        }
        OutputFormat::Full => {
            for activity in &activities {
                println!("{}", serde_json::to_string_pretty(&activity)?);
                println!("─────────────────────────────────────────");
            }
        }
    }

    Ok(())
}

/// Handle activity command with format support
pub async fn handle_activity_formatted(session_id: &str, activity_id: &str, format: &str) -> Result<()> {
    let config = load_config()?;
    let api_key = config.api_key.context("API key not configured")?;
    let client = JulesClient::new(&api_key);

    let activity = client.get_activity(session_id, activity_id).await?;

    let output_format = OutputFormat::from_str(format)?;
    match output_format {
        OutputFormat::Json | OutputFormat::Full => {
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        OutputFormat::Table => {
            let refs = vec![&activity];
            jules_core::display::print_activities_table(&refs);
        }
    }

    Ok(())
}
