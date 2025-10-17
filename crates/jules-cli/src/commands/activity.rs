//! Get single activity command implementation.

use anyhow::Result;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

/// Arguments for the activity command
pub struct ActivityArgs {
    pub session_id: String,
    pub activity_id: String,
}

/// Handle the activity command
pub async fn handle_activity(args: ActivityArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Call SDK method
    let activity = client
        .get_activity(&args.session_id, &args.activity_id)
        .await?;

    // Display the activity details
    println!("Activity Details");
    println!("================");
    println!("ID: {}", activity.id);
    println!("Type: {}", activity.activity_type());
    println!(
        "Created: {}",
        jules_core::display::display_timestamp(&activity.create_time)
    );
    println!("Originator: {}", activity.originator);

    if let Some(description) = &activity.description {
        println!("Description: {}", description);
    }

    println!();

    // Display activity-specific content
    if let Some(content) = activity.content() {
        println!("Content:");
        println!("{}", content);
        println!();
    }

    // Display artifacts if any
    if !activity.artifacts.is_empty() {
        println!("Artifacts: {} found", activity.artifacts.len());
    }

    Ok(())
}
