//! Approve plan command implementation.

use anyhow::Result;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

/// Arguments for the approve_plan command
pub struct ApprovePlanArgs {
    pub session_id: String,
}

/// Handle the approve_plan command
pub async fn handle_approve_plan(args: ApprovePlanArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Call SDK method
    client.approve_plan(&args.session_id).await?;

    // Display confirmation
    println!(
        "Plan approved successfully for session: {}",
        args.session_id
    );
    println!();
    println!("The session will now execute the approved plan.");
    println!(
        "Use 'gules session {}' to monitor progress",
        args.session_id
    );

    Ok(())
}
