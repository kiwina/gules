//! Create session command implementation.
//!
//! Creates a new Jules coding session.

use anyhow::Result;
use clap::Args;
use jules_core::{get_api_key, load_config};
use jules_rs::{
    types::session::{CreateSessionRequest, SourceContext},
    JulesClient,
};

#[derive(Args)]
pub struct CreateArgs {
    /// Session prompt/description
    pub prompt: String,

    /// Source repository ID
    #[arg(short, long)]
    pub source: String,

    /// Optional session title
    #[arg(long)]
    pub title: Option<String>,

    /// Starting branch for GitHub repos (default: main)
    #[arg(long)]
    pub branch: Option<String>,

    /// Require plan approval before execution
    #[arg(long)]
    pub require_approval: bool,

    /// Automation mode: AUTO_CREATE_PR or MANUAL
    #[arg(long, value_name = "MODE")]
    pub automation_mode: Option<String>,
}

pub async fn handle_create(args: CreateArgs) -> Result<()> {
    // Load configuration
    let config = load_config()?;

    // Get API key
    let api_key = get_api_key(None, &config)?;

    // Create client
    let client = JulesClient::new(api_key);

    // Build GitHub repo context if branch is specified
    let github_repo_context =
        args.branch
            .as_ref()
            .map(|branch| jules_rs::types::session::GitHubRepoContext {
                starting_branch: branch.clone(),
            });

    // Parse automation mode
    let automation_mode =
        args.automation_mode
            .as_ref()
            .and_then(|mode| match mode.to_uppercase().as_str() {
                "AUTO_CREATE_PR" => Some(jules_rs::types::session::AutomationMode::AutoCreatePr),
                "MANUAL" => {
                    Some(jules_rs::types::session::AutomationMode::AutomationModeUnspecified)
                }
                _ => None,
            });

    // Build request - pure SDK interface
    let request = CreateSessionRequest {
        prompt: args.prompt.clone(),
        source_context: SourceContext {
            source: args.source.clone(),
            github_repo_context,
        },
        title: args.title.clone(),
        require_plan_approval: if args.require_approval {
            Some(true)
        } else {
            None
        },
        automation_mode,
    };

    // Create session using pure SDK
    let session = client.create_session(request).await?;

    // Display success message
    println!("✅ Session created successfully!");
    println!("Session ID: {}", session.id);
    println!("Name: {}", session.name);

    if let Some(title) = &session.title {
        println!("Title: {}", title);
    }

    println!("Prompt: {}", session.prompt);

    if let Some(state) = &session.state {
        println!("Initial State: {}", state.display_name());
    }

    if let Some(url) = &session.url {
        println!("Web URL: {}", url);
    }

    println!("\nYou can now:");
    println!("  • Check status: gules session {}", session.id);
    println!("  • View activities: gules activities {}", session.id);
    println!("  • List all sessions: gules sessions");

    Ok(())
}
