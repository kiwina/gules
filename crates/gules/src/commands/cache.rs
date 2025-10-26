//! Cache management commands.
//!
//! Commands for managing the activity cache: stats, clear, delete.

use anyhow::Result;
use jules_core::activity_cache::*;

/// Show cache statistics
pub async fn handle_cache_stats() -> Result<()> {
    let stats = get_cache_stats()?;

    println!("Activity Cache Statistics");
    println!("═══════════════════════════");
    println!(
        "Status: {}",
        if stats.enabled { "Enabled" } else { "Disabled" }
    );
    println!("Location: {}", stats.cache_dir);
    println!();
    println!("Sessions: {}/{}", stats.total_sessions, stats.max_sessions);
    println!("Total Activities: {}", stats.total_activities);
    println!(
        "Disk Usage: {:.2} MiB",
        stats.total_size_bytes as f64 / 1_048_576.0
    );

    if stats.total_sessions > 0 {
        println!();
        println!("Cached Sessions:");
        let sessions = list_cached_sessions()?;
        for (i, session_id) in sessions.iter().enumerate() {
            match load_session_cache(session_id) {
                Ok(Some(cache)) => {
                    println!(
                        "  {}. {} ({} activities, updated {})",
                        i + 1,
                        session_id,
                        cache.activities.len(),
                        cache.last_updated.format("%Y-%m-%d %H:%M")
                    );
                }
                Ok(None) => {
                    // File deleted but metadata not yet updated - safe to ignore
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to load cache for session {}: {}",
                        session_id, e
                    );
                }
            }
        }
    }

    Ok(())
}

/// Clear all cache
pub async fn handle_cache_clear() -> Result<()> {
    let stats = get_cache_stats()?;

    if stats.total_sessions == 0 {
        println!("Cache is already empty.");
        return Ok(());
    }

    clear_all_cache()?;

    println!(
        "✅ Cleared cache ({} sessions, {} activities)",
        stats.total_sessions, stats.total_activities
    );

    Ok(())
}

/// Delete cache for a specific session
pub async fn handle_cache_delete(session_id: &str) -> Result<()> {
    if load_session_cache(session_id)?.is_none() {
        println!("No cache found for session: {}", session_id);
        return Ok(());
    }

    delete_session_cache(session_id)?;

    println!("✅ Deleted cache for session: {}", session_id);

    Ok(())
}
