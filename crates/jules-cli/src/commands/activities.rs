//! Activities command implementation.
//!
//! Lists all activities for a specific session.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct ActivitiesArgs {
    /// Session ID to get activities for
    pub session_id: String,

    /// Maximum number of results
    #[arg(long, default_value = "50")]
    pub limit: u32,
}

pub async fn handle_activities(args: ActivitiesArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get activities (SDK returns Response object)
    let response = client
        .list_activities(&args.session_id, Some(50), None)
        .await?;
    let activities = response.activities;

    // Apply limit
    let limited_activities: Vec<_> = activities.into_iter().take(args.limit as usize).collect();

    // Display results
    if limited_activities.is_empty() {
        println!("No activities found for session {}.", args.session_id);
        return Ok(());
    }

    println!("Session Activities ({})", limited_activities.len());
    println!("=====================");
    jules_core::display::print_activities_table(&limited_activities.iter().collect::<Vec<_>>());

    Ok(())
}
