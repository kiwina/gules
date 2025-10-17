//! Session details command implementation.
//!
//! Gets detailed information about a specific session.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct SessionArgs {
    /// Session ID to get details for
    pub id: String,
}

pub async fn handle_session(args: SessionArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get session details
    let session = client.get_session(&args.id).await?;

    // Display session details
    display_session_details(&session);

    Ok(())
}

fn display_session_details(session: &jules_rs::types::session::Session) {
    println!("Session Details");
    println!("===============");
    println!("ID: {}", session.id);
    println!("Name: {}", session.name);

    if let Some(title) = &session.title {
        println!("Title: {}", title);
    }

    println!("Prompt: {}", session.prompt);

    if let Some(state) = &session.state {
        println!("State: {}", state.display_name());
    }

    if let Some(create_time) = &session.create_time {
        println!(
            "Created: {}",
            jules_core::display::display_timestamp(create_time)
        );
    }

    if let Some(update_time) = &session.update_time {
        println!(
            "Updated: {}",
            jules_core::display::display_timestamp(update_time)
        );
    }

    if let Some(url) = &session.url {
        println!("URL: {}", url);
    }

    println!("\nSource Context:");
    println!("  Source: {}", session.source_context.source);
    if let Some(github_context) = &session.source_context.github_repo_context {
        println!("  Starting Branch: {}", github_context.starting_branch);
    }

    if !session.outputs.is_empty() {
        println!("\nOutputs:");
        for (i, output) in session.outputs.iter().enumerate() {
            println!(
                "  {}. {}",
                i + 1,
                output
                    .pull_request
                    .as_ref()
                    .map(|pr| format!("PR: {} ({})", pr.title, pr.url))
                    .unwrap_or_else(|| "No pull request".to_string())
            );
        }
    }
}
