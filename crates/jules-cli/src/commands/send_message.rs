//! Send message command implementation.

use anyhow::Result;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

/// Arguments for the send_message command
pub struct SendMessageArgs {
    pub session_id: String,
    pub message: String,
}

/// Handle the send_message command
pub async fn handle_send_message(args: SendMessageArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Call SDK method
    client.send_message(&args.session_id, &args.message).await?;

    // Display confirmation
    println!("Message sent successfully to session: {}", args.session_id);
    println!();
    println!(
        "Use 'gules session {}' to see the updated session details",
        args.session_id
    );

    Ok(())
}
