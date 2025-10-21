use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const DEFAULT_JULES_API_BASE: &str = "https://jules.googleapis.com/v1alpha";

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub api_url: Option<String>,
    #[serde(default)]
    pub default_owner: Option<String>,
    #[serde(default)]
    pub default_repo: Option<String>,
    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheConfig {
    #[serde(default = "default_cache_enabled")]
    pub enabled: bool,
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,
}

fn default_cache_enabled() -> bool {
    true
}

fn default_max_sessions() -> usize {
    50
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: default_cache_enabled(),
            max_sessions: default_max_sessions(),
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    Ok(config_dir.join("gules").join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        let config = Config::default();
        save_config(&config)?;

        println!("ℹ Created default config at: {}", config_path.display());
        println!("💡 Edit this file to set defaults (API key, default repo, etc.)");

        return Ok(config);
    }

    let contents = fs::read_to_string(&config_path).context("Failed to read config file")?;

    toml::from_str(&contents).context("Failed to parse config file")
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let contents = toml::to_string_pretty(config).context("Failed to serialize config")?;

    fs::write(&config_path, contents).context("Failed to write config file")?;

    Ok(())
}

pub fn get_api_key(cli_key: Option<String>, config: &Config) -> Result<String> {
    if let Some(key) = cli_key {
        return Ok(key);
    }

    if let Ok(key) = std::env::var("JULES_API_KEY") {
        return Ok(key);
    }

    if let Some(key) = &config.api_key {
        return Ok(key.clone());
    }

    anyhow::bail!(
        "API key not found. Set it via:\n  \
         1. --api-key flag\n  \
         2. JULES_API_KEY environment variable\n  \
         3. Edit config file: {:?}\n\n\
         Get your API key from: https://jules.google.com/settings",
        get_config_path().unwrap_or_default()
    );
}
