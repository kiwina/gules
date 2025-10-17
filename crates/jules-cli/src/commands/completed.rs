//! Completed sessions command implementation.
//!
//! Lists all completed sessions.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct CompletedArgs {
    /// Search in title and prompt
    #[arg(long)]
    pub search: Option<String>,

    /// Maximum number of results
    #[arg(long, default_value = "50")]
    pub limit: u32,
}

pub async fn handle_completed(args: CompletedArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get sessions (SDK returns Response object)
    let response = client.list_sessions(Some(50), None).await?;
    let sessions = response.sessions;

    // Filter completed sessions
    let completed_sessions: Vec<_> = sessions
        .into_iter()
        .filter(|session| {
            if let Some(ref session_state) = session.state {
                matches!(session_state, jules_rs::types::State::Completed)
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
    if completed_sessions.is_empty() {
        println!("No completed sessions found.");
        if args.search.is_some() {
            println!("Try removing the search filter to see all completed sessions.");
        }
        return Ok(());
    }

    println!("Completed Sessions ({})", completed_sessions.len());
    println!("=====================");
    jules_core::display::display_sessions_table(&completed_sessions);

    Ok(())
}
