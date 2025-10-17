//! Sessions command implementation.
//!
//! Lists all sessions with optional filtering.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct SessionsArgs {
    /// Filter by state (active, completed, failed)
    #[arg(long)]
    pub state: Option<String>,

    /// Search in title and prompt
    #[arg(long)]
    pub search: Option<String>,

    /// Maximum number of results
    #[arg(long, default_value = "50")]
    pub limit: u32,
}

pub async fn handle_sessions(args: SessionsArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get sessions (SDK returns Response object)
    let response = client.list_sessions(Some(50), None).await?;
    let sessions = response.sessions;

    // Apply filters
    let filtered_sessions: Vec<_> = sessions
        .into_iter()
        .filter(|session| {
            // State filter
            if let Some(ref state_filter) = args.state {
                if let Some(ref session_state) = session.state {
                    let state_matches = match state_filter.to_lowercase().as_str() {
                        "active" => matches!(
                            session_state,
                            jules_rs::types::State::Queued
                                | jules_rs::types::State::Planning
                                | jules_rs::types::State::AwaitingPlanApproval
                                | jules_rs::types::State::AwaitingUserFeedback
                                | jules_rs::types::State::InProgress
                        ),
                        "completed" => matches!(session_state, jules_rs::types::State::Completed),
                        "failed" => matches!(session_state, jules_rs::types::State::Failed),
                        "paused" => matches!(session_state, jules_rs::types::State::Paused),
                        _ => true, // Show all if filter doesn't match
                    };
                    if !state_matches {
                        return false;
                    }
                } else if state_filter.to_lowercase() != "unknown" {
                    // If no state and filter is not "unknown", skip
                    return false;
                }
            }

            // Search filter
            if let Some(ref search_term) = args.search {
                let search_lower = search_term.to_lowercase();
                let title_match = session
                    .title
                    .as_ref()
                    .map(|t| t.to_lowercase().contains(&search_lower))
                    .unwrap_or(false);
                let prompt_match = session.prompt.to_lowercase().contains(&search_lower);
                if !title_match && !prompt_match {
                    return false;
                }
            }

            true
        })
        .take(args.limit as usize)
        .collect();

    // Display results
    if filtered_sessions.is_empty() {
        println!("No sessions found matching the criteria.");
        return Ok(());
    }

    jules_core::display::display_sessions_table(&filtered_sessions);

    Ok(())
}
