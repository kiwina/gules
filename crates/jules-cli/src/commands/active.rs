//! Active sessions command implementation.
//!
//! Lists all active sessions (queued, planning, awaiting approval, awaiting feedback, in progress).

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct ActiveArgs {
    /// Search in title and prompt
    #[arg(long)]
    pub search: Option<String>,

    /// Maximum number of results
    #[arg(long, default_value = "50")]
    pub limit: u32,
}

pub async fn handle_active(args: ActiveArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get sessions (SDK returns Response object)
    let response = client.list_sessions(Some(50), None).await?;
    let sessions = response.sessions;

    // Filter active sessions
    let active_sessions: Vec<_> = sessions
        .into_iter()
        .filter(|session| {
            if let Some(ref session_state) = session.state {
                matches!(
                    session_state,
                    jules_rs::types::State::Queued
                        | jules_rs::types::State::Planning
                        | jules_rs::types::State::AwaitingPlanApproval
                        | jules_rs::types::State::AwaitingUserFeedback
                        | jules_rs::types::State::InProgress
                )
            } else {
                false
            }
        })
        .filter(|session| {
            // Apply search filter if provided
            if let Some(ref search_term) = args.search {
                let search_lower = search_term.to_lowercase();
                let title_match = session
                    .title
                    .as_ref()
                    .map(|t| t.to_lowercase().contains(&search_lower))
                    .unwrap_or(false);
                let prompt_match = session.prompt.to_lowercase().contains(&search_lower);
                title_match || prompt_match
            } else {
                true
            }
        })
        .take(args.limit as usize)
        .collect();

    // Display results
    if active_sessions.is_empty() {
        println!("No active sessions found.");
        if args.search.is_some() {
            println!("Try removing the search filter to see all active sessions.");
        }
        return Ok(());
    }

    println!("Active Sessions ({})", active_sessions.len());
    println!("==================");
    jules_core::display::display_sessions_table(&active_sessions);

    Ok(())
}
