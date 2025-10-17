//! Sources command implementation.
//!
//! Lists all available sources/repositories.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct SourcesArgs {
    /// Filter sources (e.g., "name:prod")
    #[arg(long)]
    pub filter: Option<String>,
    /// Maximum number of results
    #[arg(long, default_value = "50")]
    pub limit: u32,
}

pub async fn handle_sources(args: SourcesArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get sources (SDK returns Response object with all parameters exposed)
    let response = client
        .list_sources(args.filter.as_deref(), Some(args.limit), None)
        .await?;
    let sources = response.sources;

    // Display results (no additional filtering - SDK handles it)
    if sources.is_empty() {
        println!("No sources found.");
        return Ok(());
    }

    println!("Available Sources ({})", sources.len());
    println!("====================");
    jules_core::display::print_sources_table(&sources);

    Ok(())
}
