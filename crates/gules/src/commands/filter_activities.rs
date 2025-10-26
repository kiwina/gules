//! Filter activities command with caching support.
//!
//! Provides advanced filtering of session activities with local caching
//! for efficient queries and offline access.

use anyhow::{Context, Result};
use jules_core::{activity_cache::*, get_api_key, load_config};
use jules_rs::{types::activity::Activity, JulesClient};

/// Activity type filter
#[derive(Debug, Clone)]
pub enum ActivityTypeFilter {
    AgentMessage,
    UserMessage,
    Plan,
    PlanApproved,
    Progress,
    Completed,
    Failed,
}

impl ActivityTypeFilter {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "agent-message" | "agent" => Ok(Self::AgentMessage),
            "user-message" | "user" => Ok(Self::UserMessage),
            "plan" | "plan-generated" => Ok(Self::Plan),
            "plan-approved" | "approved" => Ok(Self::PlanApproved),
            "progress" | "progress-updated" => Ok(Self::Progress),
            "completed" | "session-completed" => Ok(Self::Completed),
            "failed" | "session-failed" | "error" => Ok(Self::Failed),
            _ => anyhow::bail!("Unknown activity type: {}", s),
        }
    }

    pub fn matches(&self, activity: &Activity) -> bool {
        match self {
            Self::AgentMessage => activity.agent_messaged.is_some(),
            Self::UserMessage => activity.user_messaged.is_some(),
            Self::Plan => activity.plan_generated.is_some(),
            Self::PlanApproved => activity.plan_approved.is_some(),
            Self::Progress => activity.progress_updated.is_some(),
            Self::Completed => activity.session_completed.is_some(),
            Self::Failed => activity.session_failed.is_some(),
        }
    }
}

/// Output format
#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// Native JSON output (machine-readable, complete data)
    Json,
    /// Human-readable table (truncated for display)
    Table,
    /// Full detailed view with all fields (human-readable)
    Full,
    /// Content only (just the text, no metadata)
    ContentOnly,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "table" => Ok(Self::Table),
            "full" => Ok(Self::Full),
            "content" | "content-only" => Ok(Self::ContentOnly),
            _ => anyhow::bail!(
                "Unknown output format: {}. Valid options: json, table, full, content-only",
                s
            ),
        }
    }
}

/// Filter and fetch activities with caching
pub async fn filter_activities(
    session_id: &str,
    last_n: Option<usize>,
    type_filters: Vec<ActivityTypeFilter>,
    has_bash_output: bool,
    no_cache: bool,
    output_format: OutputFormat,
) -> Result<()> {
    // Load configuration
    let config = load_config()?;
    let api_key = get_api_key(None, &config)?;
    let client = JulesClient::new(api_key);

    // Determine if caching is enabled
    let cache_enabled = config.cache.enabled && !no_cache;

    // Get activities (from cache or API)
    let activities = if cache_enabled {
        get_activities_with_cache(&client, session_id).await?
    } else {
        fetch_all_activities(&client, session_id).await?
    };

    // Apply filters
    let mut filtered = activities;

    // Filter by type
    if !type_filters.is_empty() {
        filtered.retain(|a| type_filters.iter().any(|f| f.matches(a)));
    }

    // Filter by bash output
    if has_bash_output {
        filtered.retain(|a| {
            a.artifacts
                .iter()
                .any(|artifact| artifact.bash_output.is_some())
        });
    }

    // Take last N
    if let Some(n) = last_n {
        filtered.truncate(n);
    }

    // Display results
    display_activities(&filtered, output_format)?;

    Ok(())
}

/// Get activities with caching (incremental updates)
async fn get_activities_with_cache(
    client: &JulesClient,
    session_id: &str,
) -> Result<Vec<Activity>> {
    // Try to load from cache
    let cached = load_session_cache(session_id)?;

    if let Some(cache) = cached {
        // Fetch only new activities using page token
        let response = client
            .list_activities(session_id, Some(50), cache.last_page_token.as_deref())
            .await?;

        // Update cache with new data
        let updated_cache = update_cache_incremental(session_id, &response)?;
        Ok(updated_cache.activities)
    } else {
        // No cache exists, fetch everything
        let all_activities = fetch_all_activities(client, session_id).await?;

        // Create initial cache
        let response = jules_rs::types::activity::ListActivitiesResponse {
            activities: all_activities.clone(),
            next_page_token: None,
        };

        update_cache_incremental(session_id, &response)?;
        Ok(all_activities)
    }
}

/// Display activities based on format
fn display_activities(activities: &[Activity], format: OutputFormat) -> Result<()> {
    if activities.is_empty() {
        println!("No activities found matching the filters.");
        return Ok(());
    }

    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&activities)
                .context("Failed to serialize activities")?;
            println!("{}", json);
        }
        OutputFormat::Table => {
            println!("Activities ({})", activities.len());
            println!("====================");
            let refs: Vec<&Activity> = activities.iter().collect();
            jules_core::display::print_activities_table(&refs);
        }
        OutputFormat::Full => {
            for (i, activity) in activities.iter().enumerate() {
                println!("─────────────────────────────────────────");
                println!("Activity {}/{}", i + 1, activities.len());
                println!("─────────────────────────────────────────");
                println!("ID: {}", activity.id);
                println!("Type: {}", activity.activity_type());
                println!("Time: {}", activity.create_time);
                println!("Originator: {}", activity.originator);

                if let Some(desc) = &activity.description {
                    println!("Description: {}", desc);
                }

                if let Some(content) = activity.content() {
                    println!("\nContent:");
                    println!("{}", content);
                }

                // Show artifacts
                if !activity.artifacts.is_empty() {
                    println!("\nArtifacts: {}", activity.artifacts.len());
                    for (j, artifact) in activity.artifacts.iter().enumerate() {
                        println!("  Artifact {}:", j + 1);

                        if let Some(bash) = &artifact.bash_output {
                            println!("    Type: Bash Output");
                            let command = bash.command.as_deref().unwrap_or("[Empty command]");
                            println!("    Command: {}", command);
                            let exit_status = bash
                                .exit_code
                                .map(|c| c.to_string())
                                .unwrap_or_else(|| "unknown".to_string());
                            println!("    Exit Code: {}", exit_status);
                            println!("    Output:");
                            let output = bash.output.as_deref().unwrap_or("[No output]");
                            println!("    {}", output.lines().collect::<Vec<_>>().join("\n    "));
                        }

                        if let Some(changeset) = &artifact.change_set {
                            println!("    Type: Change Set");
                            println!("    Source: {}", changeset.source);
                            if let Some(patch) = &changeset.git_patch {
                                if let Some(base_commit) = &patch.base_commit_id {
                                    println!("    Base Commit: {}", base_commit);
                                }
                                if let Some(msg) = &patch.suggested_commit_message {
                                    println!("    Suggested Commit: {}", msg);
                                }
                                if patch.unidiff_patch.is_none() {
                                    println!("    (No diff available)");
                                }
                            }
                        }
                    }
                }

                println!();
            }
        }
        OutputFormat::ContentOnly => {
            for activity in activities {
                if let Some(content) = activity.content() {
                    println!("{}", content);
                    println!("---");
                }
            }
        }
    }

    Ok(())
}
