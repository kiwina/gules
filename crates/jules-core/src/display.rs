use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::*;
use jules_rs::types::activity::{Activity, Artifact, Plan};
use jules_rs::types::session::{Session, State};
use jules_rs::types::source::Source;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub fn print_sessions_table(sessions: &[&Value]) {
    if sessions.is_empty() {
        return;
    }

    let mut max_title_len = 20;
    let id_len = 20;
    let pr_len = 6;

    for session in sessions {
        if let Some(title) = session.get("title").and_then(|v| v.as_str()) {
            max_title_len = max_title_len.max(title.len().min(50));
        }
    }

    let state_len = 11;
    let time_len = 12; // For timestamps like "2h ago" or "Oct 14, 2025"

    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );
    println!(
        "{:<width_title$} {:<20} {:<11} {:<12} {:<6}",
        "Title",
        "Session ID",
        "State",
        "Created",
        "PR",
        width_title = max_title_len
    );
    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );

    for session in sessions {
        let id = session
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let title = session
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("No title");
        let state = session
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let truncated_title = if title.len() > max_title_len {
            format!("{}...", &title[..max_title_len - 3])
        } else {
            title.to_string()
        };

        let truncated_id = if id.len() > 20 {
            format!("{}...", &id[..17])
        } else {
            id.to_string()
        };

        let state_display = parse_state_for_display(state);

        let create_time = session
            .get("createTime")
            .and_then(|v| v.as_str())
            .map(display_timestamp)
            .unwrap_or_else(|| "-".to_string());

        let has_pr = session
            .get("outputs")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().any(|o| o.get("pullRequest").is_some()))
            .unwrap_or(false);

        let pr_indicator = if has_pr {
            "✓".green().to_string()
        } else {
            "-".to_string()
        };

        println!(
            "{:<width_title$} {:<20} {:<11} {:<12} {:<6}",
            truncated_title,
            truncated_id,
            state_display,
            create_time,
            pr_indicator,
            width_title = max_title_len
        );
    }

    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );
}

pub fn display_sessions_table(sessions: &[Session]) {
    if sessions.is_empty() {
        return;
    }

    let mut max_title_len = 20;
    let id_len = 20;
    let pr_len = 6;

    for session in sessions {
        if let Some(title) = &session.title {
            max_title_len = max_title_len.max(title.len().min(50));
        }
    }

    let state_len = 11;
    let time_len = 12; // For timestamps like "2h ago" or "Oct 14, 2025"

    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );
    println!(
        "{:<width_title$} {:<20} {:<11} {:<12} {:<6}",
        "Title",
        "Session ID",
        "State",
        "Created",
        "PR",
        width_title = max_title_len
    );
    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );

    for session in sessions {
        let id = &session.id;
        let title = session.title.as_deref().unwrap_or("Untitled");

        let state_str = session
            .state
            .as_ref()
            .map(|s| format!("{:?}", s))
            .unwrap_or("unknown".to_string());

        let truncated_title = if title.len() > max_title_len {
            format!("{}...", &title[..max_title_len - 3])
        } else {
            title.to_string()
        };

        let truncated_id = if id.len() > 20 {
            format!("{}...", &id[..17])
        } else {
            id.to_string()
        };

        let state_display = parse_state_for_display(&state_str);

        let create_time = session
            .create_time
            .as_ref()
            .map(|t| display_timestamp(t))
            .unwrap_or_else(|| "-".to_string());

        // For now, assume no PR info in Session struct
        let pr_indicator = "-".to_string();

        println!(
            "{:<width_title$} {:<20} {:<11} {:<12} {:<6}",
            truncated_title,
            truncated_id,
            state_display,
            create_time,
            pr_indicator,
            width_title = max_title_len
        );
    }

    println!(
        "{}",
        "─".repeat(max_title_len + id_len + state_len + time_len + pr_len + 13)
    );
}

pub async fn save_response(
    response: &Value,
    output_path: Option<PathBuf>,
    pretty: bool,
) -> Result<()> {
    let json_str = if pretty {
        serde_json::to_string_pretty(response)?
    } else {
        serde_json::to_string(response)?
    };

    if let Some(path) = output_path {
        fs::write(&path, &json_str)
            .with_context(|| format!("Failed to write to file: {}", path.display()))?;
        println!("{} Response saved to: {}", "✓".green(), path.display());
    } else {
        println!("{}", json_str);
    }

    Ok(())
}
fn parse_state_for_display(state_str: &str) -> colored::ColoredString {
    // Parse the state string into State enum
    let state = match state_str {
        "STATE_UNSPECIFIED" => State::StateUnspecified,
        "QUEUED" => State::Queued,
        "PLANNING" => State::Planning,
        "AWAITING_PLAN_APPROVAL" => State::AwaitingPlanApproval,
        "AWAITING_USER_FEEDBACK" => State::AwaitingUserFeedback,
        "IN_PROGRESS" => State::InProgress,
        "PAUSED" => State::Paused,
        "FAILED" => State::Failed,
        "COMPLETED" => State::Completed,
        _ => {
            // Unknown state, return as plain text
            return state_str.normal();
        }
    };

    // Return colored display based on state
    match state {
        State::StateUnspecified => state_str.normal(),
        State::Queued => state_str.cyan().bold(),
        State::Planning => state_str.yellow().bold(),
        State::AwaitingPlanApproval => state_str.magenta().bold(),
        State::AwaitingUserFeedback => state_str.blue().bold(),
        State::InProgress => state_str.yellow().bold(),
        State::Paused => state_str.white().dimmed().bold(),
        State::Failed => state_str.red().bold(),
        State::Completed => state_str.green().bold(),
    }
}

/// Display timestamp in human-readable format for CLI (concise) - use ISO format in JSON
pub fn display_timestamp(timestamp: &str) -> String {
    match DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => {
            let utc_dt = dt.with_timezone(&Utc);
            let now = Utc::now();
            let duration = now.signed_duration_since(utc_dt);

            if duration.num_seconds() < 60 {
                "Just now".to_string()
            } else if duration.num_minutes() < 60 {
                format!("{}m ago", duration.num_minutes())
            } else if duration.num_hours() < 24 {
                format!("{}h ago", duration.num_hours())
            } else if duration.num_days() < 7 {
                format!("{}d ago", duration.num_days())
            } else {
                // For older timestamps, show date
                utc_dt.format("%b %d, %Y").to_string()
            }
        }
        Err(_) => {
            // If parsing fails, return the original timestamp
            timestamp.to_string()
        }
    }
}

/// Display activity summary for CLI (concise) - use JSON for full details
pub fn display_activity_summary(activity: &Activity) {
    match activity.activity_type() {
        "Agent Message" => {
            if let Some(msg) = &activity.agent_messaged {
                // Truncate long messages for CLI
                let preview = if msg.agent_message.len() > 80 {
                    format!("{}...", &msg.agent_message[..77])
                } else {
                    msg.agent_message.clone()
                };
                println!("{} Agent: {}", "💬".blue(), preview);
            }
        }
        "User Message" => {
            if let Some(msg) = &activity.user_messaged {
                // Truncate long messages for CLI
                let preview = if msg.user_message.len() > 80 {
                    format!("{}...", &msg.user_message[..77])
                } else {
                    msg.user_message.clone()
                };
                println!("{} User: {}", "👤".green(), preview);
            }
        }
        "Progress Update" => {
            if let Some(progress) = &activity.progress_updated {
                // Show only title, not full description (too verbose for CLI)
                println!("{} {}", "⚙".blue(), progress.title);

                // Show artifact summaries if present
                for artifact in &activity.artifacts {
                    display_artifact_summary(artifact);
                }
            }
        }
        "Plan Generated" => {
            if let Some(plan_generated) = &activity.plan_generated {
                display_plan_summary(&plan_generated.plan);
            } else {
                println!("{} {}", "📋".yellow(), "Plan generated".bold());
            }
        }
        "Plan Approved" => {
            println!("{} {}", "✓".green(), "Plan approved".bold());
        }
        "Session Completed" => {
            println!("{} {}", "✓".green(), "Session completed".bold());
        }
        "Session Failed" => {
            if let Some(failed) = &activity.session_failed {
                println!("{} Session failed: {}", "✗".red(), failed.reason);
            } else {
                println!("{} {}", "✗".red(), "Session failed".bold());
            }
        }
        _ => {
            // Show activity type for unknown types
            println!("{} {}", "❓".dimmed(), activity.activity_type().dimmed());
        }
    }
}

/// Display plan summary for CLI (concise) - use JSON for full details
pub fn display_plan_summary(plan: &Plan) {
    println!("{} Plan with {} steps", "📋".yellow(), plan.steps.len());

    // Show first 3 step titles for context (truncated if long)
    for (i, step) in plan.steps.iter().enumerate().take(3) {
        let step_num = i + 1;
        let title_preview = if step.title.len() > 60 {
            format!("{}...", &step.title[..57])
        } else {
            step.title.clone()
        };
        println!("  {}. {}", step_num, title_preview.dimmed());
    }

    // If there are more steps, indicate truncation
    if plan.steps.len() > 3 {
        let remaining = plan.steps.len() - 3;
        println!("  ... and {} more steps", remaining.to_string().dimmed());
    }
}

/// Display artifact summary for CLI (concise) - use JSON for full content
pub fn display_artifact_summary(artifact: &Artifact) {
    if let Some(bash) = &artifact.bash_output {
        // Show command and exit code, truncate long commands
        let cmd_preview = if bash.command.len() > 50 {
            format!("{}...", &bash.command[..47])
        } else {
            bash.command.clone()
        };
        println!(
            "  {} {} (exit: {})",
            "🐚".cyan(),
            cmd_preview,
            bash.exit_code
        );
    }

    if let Some(change_set) = &artifact.change_set {
        if let Some(patch) = &change_set.git_patch {
            // Count lines added/removed from git patch
            let added = patch.unidiff_patch.matches("+\n").count();
            let removed = patch.unidiff_patch.matches("-\n").count();
            println!(
                "  {} Git patch: {} added, {} removed",
                "📝".yellow(),
                added,
                removed
            );
        }
    }

    if let Some(media) = &artifact.media {
        println!("  {} Media: {}", "🖼".purple(), media.mime_type);
    }
}

/// Print sources in a formatted table
pub fn print_sources_table(sources: &[Source]) {
    if sources.is_empty() {
        return;
    }

    let owner_len = 15;
    let repo_len = 25;
    let id_len = 30;
    let private_len = 7;
    let branches_len = 9;

    println!(
        "{}",
        "─".repeat(owner_len + repo_len + id_len + private_len + branches_len + 13)
    );
    println!(
        "{:<width_owner$} {:<width_repo$} {:<width_id$} {:<width_private$} {:<width_branches$}",
        "Owner",
        "Repository",
        "Source ID",
        "Private",
        "Branches",
        width_owner = owner_len,
        width_repo = repo_len,
        width_id = id_len,
        width_private = private_len,
        width_branches = branches_len
    );
    println!(
        "{}",
        "─".repeat(owner_len + repo_len + id_len + private_len + branches_len + 13)
    );

    for source in sources {
        let (owner, repo, is_private, branches_count) =
            if let Some(github_repo) = &source.github_repo {
                (
                    github_repo.owner.clone(),
                    github_repo.repo.clone(),
                    github_repo.is_private.unwrap_or(false),
                    github_repo.branches.len(),
                )
            } else {
                ("unknown".to_string(), "unknown".to_string(), false, 0)
            };

        let truncated_owner = if owner.len() > owner_len {
            format!("{}...", &owner[..owner_len - 3])
        } else {
            owner
        };

        let truncated_repo = if repo.len() > repo_len {
            format!("{}...", &repo[..repo_len - 3])
        } else {
            repo
        };

        let truncated_id = if source.id.len() > id_len {
            format!("{}...", &source.id[..id_len - 3])
        } else {
            source.id.clone()
        };

        let private_display = if is_private {
            "Yes".red()
        } else {
            "No".green()
        };

        println!(
            "{:<width_owner$} {:<width_repo$} {:<width_id$} {:<width_private$} {:<width_branches$}",
            truncated_owner,
            truncated_repo,
            truncated_id,
            private_display,
            branches_count,
            width_owner = owner_len,
            width_repo = repo_len,
            width_id = id_len,
            width_private = private_len,
            width_branches = branches_len
        );
    }

    println!(
        "{}",
        "─".repeat(owner_len + repo_len + id_len + private_len + branches_len + 13)
    );
}

pub fn print_activities_table(activities: &[&Activity]) {
    if activities.is_empty() {
        return;
    }

    let id_len = 20;
    let type_len = 15;
    let time_len = 12;
    let content_len = 40;

    println!(
        "{}",
        "─".repeat(id_len + type_len + time_len + content_len + 13)
    );
    println!(
        "{:<width_id$} {:<width_type$} {:<width_time$} {:<width_content$}",
        "Activity ID",
        "Type",
        "Time",
        "Content",
        width_id = id_len,
        width_type = type_len,
        width_time = time_len,
        width_content = content_len
    );
    println!(
        "{}",
        "─".repeat(id_len + type_len + time_len + content_len + 13)
    );

    for activity in activities {
        let truncated_id = if activity.id.len() > id_len {
            format!("{}...", &activity.id[..id_len - 3])
        } else {
            activity.id.clone()
        };

        let activity_type = activity.activity_type();
        let truncated_type = if activity_type.len() > type_len {
            format!("{}...", &activity_type[..type_len - 3])
        } else {
            activity_type.to_string()
        };

        let time_display = display_timestamp(&activity.create_time);

        let content_preview = activity.content().unwrap_or_else(|| "-".to_string());
        let truncated_content = if content_preview.len() > content_len {
            format!("{}...", &content_preview[..content_len - 3])
        } else {
            content_preview
        };

        println!(
            "{:<width_id$} {:<width_type$} {:<width_time$} {:<width_content$}",
            truncated_id,
            truncated_type,
            time_display,
            truncated_content,
            width_id = id_len,
            width_type = type_len,
            width_time = time_len,
            width_content = content_len
        );
    }

    println!(
        "{}",
        "─".repeat(id_len + type_len + time_len + content_len + 13)
    );
}
