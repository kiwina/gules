//! Config command implementations.
//!
//! Manages Jules CLI configuration.

use anyhow::Result;
use clap::Args;
use jules_core::{get_config_path, load_config, save_config, Config};

#[derive(Args)]
pub struct ConfigShowArgs;

pub async fn handle_config_show(_args: ConfigShowArgs) -> Result<()> {
    let config = load_config()?;

    println!("Current Configuration");
    println!("=====================");
    println!(
        "API Key: {}",
        if config.api_key.is_some() {
            "Set"
        } else {
            "Not set"
        }
    );
    println!(
        "API URL: {}",
        config.api_url.as_deref().unwrap_or("Default")
    );
    println!(
        "Default Owner: {}",
        config.default_owner.as_deref().unwrap_or("Not set")
    );
    println!(
        "Default Repo: {}",
        config.default_repo.as_deref().unwrap_or("Not set")
    );

    let config_file = get_config_path()?;
    println!("Config file: {}", config_file.display());

    Ok(())
}

#[derive(Args)]
pub struct ConfigInitArgs;

pub async fn handle_config_init(_args: ConfigInitArgs) -> Result<()> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        println!(
            "Configuration file already exists at: {}",
            config_path.display()
        );
        println!("Use 'gules config show' to view current settings.");
        return Ok(());
    }

    // Create default config
    let config = Config::default();
    save_config(&config)?;

    println!(
        "✅ Configuration file initialized at: {}",
        config_path.display()
    );
    println!("You can now set your API key with: gules config set api_key YOUR_API_KEY");

    Ok(())
}

#[derive(Args)]
pub struct ConfigSetArgs {
    /// Configuration key to set
    pub key: String,
    /// Value to set
    pub value: String,
}

pub async fn handle_config_set(args: ConfigSetArgs) -> Result<()> {
    let mut config = load_config()?;

    match args.key.as_str() {
        "api_key" => {
            config.api_key = Some(args.value.clone());
            println!("✅ API key set successfully");
        }
        "api_url" => {
            config.api_url = Some(args.value.clone());
            println!("✅ API URL set to: {}", args.value);
        }
        "default_owner" => {
            config.default_owner = Some(args.value.clone());
            println!("✅ Default owner set to: {}", args.value);
        }
        "default_repo" => {
            config.default_repo = Some(args.value.clone());
            println!("✅ Default repo set to: {}", args.value);
        }
        _ => {
            anyhow::bail!("Unknown configuration key: {}. Supported keys: api_key, api_url, default_owner, default_repo", args.key);
        }
    }

    save_config(&config)?;
    Ok(())
}
