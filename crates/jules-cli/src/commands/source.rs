//! Source details command implementation.
//!
//! Gets detailed information about a specific source/repository.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::JulesClient;

#[derive(Args)]
pub struct SourceArgs {
    /// Source ID to get details for
    pub id: String,
}

pub async fn handle_source(args: SourceArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Get source details
    let source = client.get_source(&args.id).await?;

    // Display source details
    display_source_details(&source);

    Ok(())
}

fn display_source_details(source: &jules_rs::types::source::Source) {
    println!("Source Details");
    println!("==============");
    println!("ID: {}", source.id);
    println!("Name: {}", source.name);

    if let Some(github_repo) = &source.github_repo {
        println!("\nGitHub Repository:");
        println!("  Owner: {}", github_repo.owner);
        println!("  Repository: {}", github_repo.repo);

        if let Some(is_private) = github_repo.is_private {
            println!("  Private: {}", if is_private { "Yes" } else { "No" });
        }

        if let Some(default_branch) = &github_repo.default_branch {
            println!("  Default Branch: {}", default_branch.display_name);
        }

        println!("  Branches: {}", github_repo.branches.len());
        if !github_repo.branches.is_empty() {
            println!("  Branch List:");
            for branch in &github_repo.branches {
                println!("    - {}", branch.display_name);
            }
        }
    }
}
