//! Activity cache management for efficient filtering and pagination.
//!
//! This module provides local caching of Jules session activities to enable:
//! - Fast filtering without repeated API calls
//! - Incremental updates using page tokens
//! - FIFO eviction when max sessions reached
//! - Cache management (clear, delete specific sessions)

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use jules_rs::types::activity::{Activity, ListActivitiesResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Maximum number of activities to fetch from API
const MAX_ACTIVITIES_TO_FETCH: usize = 100;
/// Page size for API pagination
const ACTIVITIES_PAGE_SIZE: u32 = 50;

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityCacheConfig {
    /// Enable activity caching
    pub enabled: bool,
    /// Maximum number of sessions to cache (FIFO eviction)
    pub max_sessions: usize,
}

impl Default for ActivityCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_sessions: 50,
        }
    }
}

/// Cached session activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCache {
    /// Session ID
    pub session_id: String,
    /// All cached activities (ordered by creation time, newest first)
    pub activities: Vec<Activity>,
    /// Last page token used (for incremental updates)
    pub last_page_token: Option<String>,
    /// When this cache was last updated
    pub last_updated: DateTime<Utc>,
    /// When this cache was first created
    pub created_at: DateTime<Utc>,
}

/// Cache metadata for FIFO eviction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// Session IDs in order of first access (oldest first)
    pub access_order: Vec<String>,
    /// Cache configuration
    pub config: ActivityCacheConfig,
}

impl Default for CacheMetadata {
    fn default() -> Self {
        Self {
            access_order: Vec::new(),
            config: ActivityCacheConfig::default(),
        }
    }
}

/// Get the cache directory path
pub fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine cache directory"))?;
    Ok(cache_dir.join("gules").join("activities"))
}

/// Get cache file path for a session
fn get_session_cache_path(session_id: &str) -> Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
    Ok(cache_dir.join(format!("{}.json", session_id)))
}

/// Get metadata file path
fn get_metadata_path() -> Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
    Ok(cache_dir.join("metadata.json"))
}

/// Load cache metadata
pub fn load_metadata() -> Result<CacheMetadata> {
    let metadata_path = get_metadata_path()?;

    if !metadata_path.exists() {
        let metadata = CacheMetadata::default();
        save_metadata(&metadata)?;
        return Ok(metadata);
    }

    let contents = fs::read_to_string(&metadata_path).context("Failed to read metadata")?;
    serde_json::from_str(&contents).context("Failed to parse metadata")
}

/// Save cache metadata
pub fn save_metadata(metadata: &CacheMetadata) -> Result<()> {
    let metadata_path = get_metadata_path()?;
    let contents = serde_json::to_string_pretty(metadata).context("Failed to serialize metadata")?;
    fs::write(&metadata_path, contents).context("Failed to write metadata")?;
    Ok(())
}

/// Load cached activities for a session
pub fn load_session_cache(session_id: &str) -> Result<Option<SessionCache>> {
    let cache_path = get_session_cache_path(session_id)?;

    if !cache_path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&cache_path)
        .context(format!("Failed to read cache for session {}", session_id))?;

    let cache: SessionCache = serde_json::from_str(&contents)
        .context(format!("Failed to parse cache for session {}", session_id))?;

    Ok(Some(cache))
}

/// Save cached activities for a session
pub fn save_session_cache(cache: &SessionCache) -> Result<()> {
    let cache_path = get_session_cache_path(&cache.session_id)?;
    let contents =
        serde_json::to_string_pretty(cache).context("Failed to serialize session cache")?;
    fs::write(&cache_path, contents).context("Failed to write session cache")?;

    // Update metadata access order
    let mut metadata = load_metadata()?;

    // Remove from current position (if exists)
    metadata
        .access_order
        .retain(|id| id != &cache.session_id);

    // Add to end (most recently accessed)
    metadata.access_order.push(cache.session_id.clone());

    // FIFO eviction if needed
    if metadata.access_order.len() > metadata.config.max_sessions {
        let to_remove_count = metadata.access_order.len() - metadata.config.max_sessions;
        let evicted_sessions: Vec<String> = metadata.access_order.drain(..to_remove_count).collect();
        for session_id in evicted_sessions {
            let cache_path = get_session_cache_path(&session_id)?;
            if cache_path.exists() {
                fs::remove_file(&cache_path)
                    .context(format!("Failed to delete evicted cache for session {}", session_id))?;
            }
        }
    }

    save_metadata(&metadata)?;
    Ok(())
}

/// Delete cache for a specific session
pub fn delete_session_cache(session_id: &str) -> Result<()> {
    let cache_path = get_session_cache_path(session_id)?;

    if cache_path.exists() {
        fs::remove_file(&cache_path)
            .context(format!("Failed to delete cache for session {}", session_id))?;
    }

    // Update metadata
    let mut metadata = load_metadata()?;
    metadata.access_order.retain(|id| id != session_id);
    save_metadata(&metadata)?;

    Ok(())
}

/// Clear all cached activities
pub fn clear_all_cache() -> Result<()> {
    let cache_dir = get_cache_dir()?;

    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).context("Failed to clear cache directory")?;
    }

    // Recreate empty cache
    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
    save_metadata(&CacheMetadata::default())?;

    Ok(())
}

/// List all cached session IDs
pub fn list_cached_sessions() -> Result<Vec<String>> {
    let metadata = load_metadata()?;
    Ok(metadata.access_order.clone())
}

/// Get cache statistics
pub fn get_cache_stats() -> Result<CacheStats> {
    let metadata = load_metadata()?;
    let cache_dir = get_cache_dir()?;

    let mut total_activities = 0;
    let mut total_size_bytes = 0u64;

    for session_id in &metadata.access_order {
        if let Ok(Some(cache)) = load_session_cache(session_id) {
            total_activities += cache.activities.len();
        }

        if let Ok(path) = get_session_cache_path(session_id) {
            if let Ok(meta) = fs::metadata(&path) {
                total_size_bytes += meta.len();
            }
        }
    }

    Ok(CacheStats {
        enabled: metadata.config.enabled,
        total_sessions: metadata.access_order.len(),
        max_sessions: metadata.config.max_sessions,
        total_activities,
        total_size_bytes,
        cache_dir: cache_dir.display().to_string(),
    })
}

/// Cache statistics
#[derive(Debug)]
pub struct CacheStats {
    pub enabled: bool,
    pub total_sessions: usize,
    pub max_sessions: usize,
    pub total_activities: usize,
    pub total_size_bytes: u64,
    pub cache_dir: String,
}

/// Merge new activities into cache (deduplication by ID)
pub fn merge_activities(
    existing: Vec<Activity>,
    new_activities: Vec<Activity>,
) -> Vec<Activity> {
    let mut merged: HashMap<String, Activity> = HashMap::new();

    // Add existing activities
    for activity in existing {
        merged.insert(activity.id.clone(), activity);
    }

    // Add/update with new activities
    for activity in new_activities {
        merged.insert(activity.id.clone(), activity);
    }

    // Convert back to vec and sort by creation time (newest first)
    let mut result: Vec<Activity> = merged.into_values().collect();
    result.sort_by(|a, b| b.create_time.cmp(&a.create_time));

    result
}

/// Update cache with new API response (incremental update)
pub fn update_cache_incremental(
    session_id: &str,
    response: &ListActivitiesResponse,
) -> Result<SessionCache> {
    let now = Utc::now();

    let mut cache = load_session_cache(session_id)?.unwrap_or_else(|| SessionCache {
        session_id: session_id.to_string(),
        activities: Vec::new(),
        last_page_token: None,
        last_updated: now,
        created_at: now,
    });

    // Merge new activities (deduplication)
    cache.activities = merge_activities(cache.activities, response.activities.clone());

    // Update metadata
    cache.last_page_token = response.next_page_token.clone();
    cache.last_updated = now;

    // Save to disk
    save_session_cache(&cache)?;

    Ok(cache)
}

/// Fetch all activities with pagination (up to MAX_ACTIVITIES_TO_FETCH)
pub async fn fetch_all_activities(
    client: &jules_rs::JulesClient,
    session_id: &str,
) -> Result<Vec<Activity>> {
    let mut all_activities = Vec::new();
    let mut page_token: Option<String> = None;

    // Fetch up to MAX_ACTIVITIES_TO_FETCH activities total
    while all_activities.len() < MAX_ACTIVITIES_TO_FETCH {
        let response = client
            .list_activities(session_id, Some(ACTIVITIES_PAGE_SIZE), page_token.as_deref())
            .await?;

        all_activities.extend(response.activities);

        // Check if there's more data
        if response.next_page_token.is_none() || all_activities.len() >= 100 {
            break;
        }

        page_token = response.next_page_token;
    }

    // Sort by creation time (newest first)
    all_activities.sort_by(|a, b| b.create_time.cmp(&a.create_time));

    Ok(all_activities)
}
