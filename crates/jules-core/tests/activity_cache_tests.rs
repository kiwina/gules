//! Tests for activity cache functionality.

use chrono::Utc;
use jules_core::activity_cache::*;
use jules_rs::types::activity::{Activity, ListActivitiesResponse};

/// Helper to create a test activity
fn create_test_activity(id: &str, session_failed: bool) -> Activity {
    Activity {
        name: format!("sessions/test/activities/{}", id),
        id: id.to_string(),
        description: Some(format!("Test activity {}", id)),
        create_time: Utc::now().to_rfc3339(),
        originator: "test".to_string(),
        artifacts: vec![],
        agent_messaged: None,
        user_messaged: None,
        plan_generated: None,
        plan_approved: None,
        progress_updated: None,
        session_completed: None,
        session_failed: if session_failed {
            Some(jules_rs::types::activity::SessionFailed {
                reason: "Test failure".to_string(),
            })
        } else {
            None
        },
    }
}

#[test]
fn test_cache_config_defaults() {
    let config = ActivityCacheConfig::default();
    assert!(config.enabled);
    assert_eq!(config.max_sessions, 50);
}

#[test]
fn test_merge_activities_deduplication() {
    let activity1 = create_test_activity("1", false);
    let activity2 = create_test_activity("2", false);
    let activity3 = create_test_activity("1", true); // Duplicate ID

    let existing = vec![activity1.clone(), activity2.clone()];
    let new = vec![activity3.clone()];

    let merged = merge_activities(existing, new);

    // Should have 2 activities (deduplication)
    assert_eq!(merged.len(), 2);

    // ID "1" should be updated to the new version
    let updated = merged.iter().find(|a| a.id == "1").unwrap();
    assert!(updated.session_failed.is_some());
}

#[test]
fn test_merge_activities_sorting() {
    use std::thread::sleep;
    use std::time::Duration;

    let mut activity1 = create_test_activity("1", false);
    sleep(Duration::from_millis(10));
    let mut activity2 = create_test_activity("2", false);
    sleep(Duration::from_millis(10));
    let mut activity3 = create_test_activity("3", false);

    // Create timestamps in specific order (oldest to newest)
    activity1.create_time = (Utc::now() - chrono::Duration::seconds(30)).to_rfc3339();
    activity2.create_time = (Utc::now() - chrono::Duration::seconds(20)).to_rfc3339();
    activity3.create_time = (Utc::now() - chrono::Duration::seconds(10)).to_rfc3339();

    let merged = merge_activities(vec![activity1], vec![activity2, activity3]);

    // Should be sorted by creation time (newest first)
    assert_eq!(merged.len(), 3);
    assert_eq!(merged[0].id, "3"); // Newest
    assert_eq!(merged[1].id, "2");
    assert_eq!(merged[2].id, "1"); // Oldest
}

// Note: File I/O tests (save, load, clear, etc.) are skipped
// because they require mocking the cache directory or actual filesystem access.
// These are better tested through integration tests or manual testing.
//
// The core logic (merge_activities, deduplication, sorting) is tested above.
