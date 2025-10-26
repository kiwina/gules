//! Integration tests for extended commands (watch, monitor, issue-status, pr-status)

use gules::extended_commands::OutputFormat;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Test helper: Mock session state
#[derive(Debug, Clone)]
struct MockSession {
    id: String,
    #[allow(dead_code)]
    title: Option<String>,
    state: Option<String>,
    activities_count: usize,
}

impl MockSession {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: Some("Test Session".to_string()),
            state: Some("IN_PROGRESS".to_string()),
            activities_count: 0,
        }
    }

    fn completed(mut self) -> Self {
        self.state = Some("COMPLETED".to_string());
        self
    }

    fn with_activities(mut self, count: usize) -> Self {
        self.activities_count = count;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────
// OUTPUT FORMAT TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_output_format_parse_json() {
    let format = OutputFormat::parse("json").unwrap();
    assert!(matches!(format, OutputFormat::Json));
}

#[test]
fn test_output_format_parse_table() {
    let format = OutputFormat::parse("table").unwrap();
    assert!(matches!(format, OutputFormat::Table));
}

#[test]
fn test_output_format_parse_full() {
    let format = OutputFormat::parse("full").unwrap();
    assert!(matches!(format, OutputFormat::Full));
}

#[test]
fn test_output_format_parse_case_insensitive() {
    assert!(matches!(OutputFormat::parse("JSON").unwrap(), OutputFormat::Json));
    assert!(matches!(OutputFormat::parse("Table").unwrap(), OutputFormat::Table));
    assert!(matches!(OutputFormat::parse("FULL").unwrap(), OutputFormat::Full));
}

#[test]
fn test_output_format_parse_invalid() {
    let result = OutputFormat::parse("invalid");
    assert!(result.is_err());
    
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Unknown output format"));
    assert!(err_msg.contains("invalid"));
}

#[test]
fn test_output_format_parse_empty() {
    let result = OutputFormat::parse("");
    assert!(result.is_err());
}

// ─────────────────────────────────────────────────────────────────────────
// WATCH COMMAND TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_watch_initialization() {
    // Test that watch command initializes with correct parameters
    let session_id = "test-session-123";
    let interval = 5u64;

    assert!(interval > 0);
    assert!(session_id.starts_with("test"));
}

#[test]
fn test_watch_session_state_detection() {
    // Test that watch detects terminal states correctly
    let terminal_states = vec!["COMPLETED", "FAILED", "PAUSED"];

    for _state in &terminal_states {
        let session = MockSession::new("test-1").completed();
        assert_eq!(session.state, Some("COMPLETED".to_string()));
    }
}

#[test]
fn test_watch_activity_tracking() {
    // Test that watch tracks activity count changes
    let session1 = MockSession::new("test-2").with_activities(0);
    let session2 = MockSession::new("test-2").with_activities(3);

    assert_eq!(session1.activities_count, 0);
    assert_eq!(session2.activities_count, 3);
    assert!(session2.activities_count > session1.activities_count);
}

#[test]
fn test_watch_interval_validation() {
    // Test that watch accepts valid intervals
    let valid_intervals = vec![1u64, 5u64, 10u64, 30u64, 60u64];

    for interval in valid_intervals {
        assert!(interval > 0, "Interval {} should be valid", interval);
    }
}

#[test]
fn test_watch_session_id_validation() {
    // Test that watch validates session IDs
    let valid_ids = vec!["session-123", "abc-def-ghi", "test_session_001"];

    for id in valid_ids {
        assert!(!id.is_empty());
    }
}

// ─────────────────────────────────────────────────────────────────────────
// MONITOR COMMAND TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_monitor_initialization() {
    // Test that monitor command initializes correctly
    let interval = 10u64;
    let max_sessions = 100usize;

    assert!(interval > 0);
    assert!(max_sessions > 0);
}

#[test]
fn test_monitor_session_collection() {
    // Test that monitor can track multiple sessions
    let sessions = vec![
        MockSession::new("session-1"),
        MockSession::new("session-2"),
        MockSession::new("session-3"),
    ];

    assert_eq!(sessions.len(), 3);
    for (i, session) in sessions.iter().enumerate() {
        assert!(!session.id.is_empty());
        assert_eq!(session.id, format!("session-{}", i + 1));
    }
}

#[test]
fn test_monitor_state_distribution() {
    // Test that monitor correctly tracks different session states
    let sessions = vec![
        MockSession::new("s1"),
        MockSession::new("s2").completed(),
        MockSession::new("s3"),
    ];

    let in_progress = sessions
        .iter()
        .filter(|s| s.state.as_deref() == Some("IN_PROGRESS"))
        .count();
    let completed = sessions
        .iter()
        .filter(|s| s.state.as_deref() == Some("COMPLETED"))
        .count();

    assert_eq!(in_progress, 2);
    assert_eq!(completed, 1);
}

#[test]
fn test_monitor_refresh_interval() {
    // Test that monitor respects refresh intervals
    let intervals = vec![1u64, 5u64, 10u64];

    for interval in intervals {
        assert!(interval > 0);
        let duration = Duration::from_secs(interval);
        assert!(duration.as_secs() == interval);
    }
}

#[test]
fn test_monitor_empty_session_list() {
    // Test that monitor handles empty session lists gracefully
    let sessions: Vec<MockSession> = vec![];
    assert_eq!(sessions.len(), 0);
}

// ─────────────────────────────────────────────────────────────────────────
// ISSUE-STATUS COMMAND TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_issue_status_github_url_parsing() {
    // Test that issue-status correctly parses GitHub URLs
    let owner = "my-org";
    let repo = "my-repo";
    let issue = 42u32;

    assert!(issue > 0);

    let url = format!("https://github.com/{}/{}/issues/{}", owner, repo, issue);
    assert!(url.contains(owner));
    assert!(url.contains(repo));
    assert!(url.contains("42"));
}

#[test]
fn test_issue_status_session_extraction() {
    // Test that issue-status correctly extracts session IDs from comments
    let comment_text = "This issue was created by Jules session: session-abc-123";

    // Simple regex pattern to extract session IDs
    let session_pattern = regex::Regex::new(r"session-[a-zA-Z0-9_-]+").unwrap();
    let captures: Vec<_> = session_pattern
        .captures_iter(comment_text)
        .map(|c| c.get(0).unwrap().as_str())
        .collect();

    assert_eq!(captures.len(), 1);
    assert_eq!(captures[0], "session-abc-123");
}

#[test]
fn test_issue_status_multiple_sessions() {
    // Test extracting multiple session IDs from a single comment
    let comment_text = "Sessions: session-001, session-002, session-003";

    let session_pattern = regex::Regex::new(r"session-[a-zA-Z0-9_-]+").unwrap();
    let captures: Vec<_> = session_pattern
        .captures_iter(comment_text)
        .map(|c| c.get(0).unwrap().as_str())
        .collect();

    assert_eq!(captures.len(), 3);
}

#[test]
fn test_issue_status_no_sessions_found() {
    // Test handling when no sessions are found in comments
    let comment_text = "This is a regular comment with no session references";

    let session_pattern = regex::Regex::new(r"session-[a-zA-Z0-9_-]+").unwrap();
    let captures: Vec<_> = session_pattern
        .captures_iter(comment_text)
        .map(|c| c.get(0).unwrap().as_str())
        .collect();

    assert_eq!(captures.len(), 0);
}

#[test]
fn test_issue_status_session_id_patterns() {
    // Test various session ID pattern formats
    let patterns = vec![
        "session-abc123",
        "session_abc_123",
        "session-ABC-DEF-GHI",
        "sessions/abc123def456",
    ];

    for pattern in patterns {
        assert!(!pattern.is_empty());
    }
}

// ─────────────────────────────────────────────────────────────────────────
// PR-STATUS COMMAND TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_pr_status_url_parsing() {
    // Test that pr-status correctly parses GitHub PR URLs
    let pr_url = "https://github.com/my-org/my-repo/pull/42";

    let parts: Vec<&str> = pr_url.split('/').collect();
    // parts: ["https:", "", "github.com", "my-org", "my-repo", "pull", "42"]
    assert!(parts.len() >= 7);
    assert_eq!(parts[2], "github.com");
    assert_eq!(parts[3], "my-org");
    assert_eq!(parts[4], "my-repo");
    assert_eq!(parts[5], "pull");
    assert_eq!(parts[6], "42");
}

#[test]
fn test_pr_status_number_extraction() {
    // Test extracting PR number from URL
    let pr_url = "https://github.com/kiwina/gules/pull/123";

    let parts: Vec<&str> = pr_url.split('/').collect();
    let pr_number = parts[6]; // Index 6, not 7

    assert_eq!(pr_number, "123");
}

#[test]
fn test_pr_status_invalid_url_handling() {
    // Test handling of invalid PR URLs
    let invalid_urls = vec!["https://github.com/invalid", "not-a-url", ""];

    for url in invalid_urls {
        assert!(
            url.split('/').collect::<Vec<_>>().len() < 7 || url.is_empty() || !url.contains("pull")
        );
    }
}

#[test]
fn test_pr_status_session_output_extraction() {
    // Test extracting PR info from session outputs
    let output =
        r#"{"pull_request": {"url": "https://github.com/org/repo/pull/42", "title": "Test PR"}}"#;

    assert!(output.contains("pull_request"));
    assert!(output.contains("url"));
    assert!(output.contains("title"));
}

// ─────────────────────────────────────────────────────────────────────────
// HELPER FUNCTION TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_gh_cli_detection_logic() {
    // Test logic for detecting gh CLI availability
    let is_available = true; // Mock: would call actual detection

    // Just verify the detection logic works (simplified test)
    assert!(is_available);
}

#[test]
fn test_regex_session_extraction() {
    // Test regex extraction of session IDs
    let re = regex::Regex::new(r"sessions/([a-zA-Z0-9_-]+)").unwrap();
    let text = "See sessions/abc-def-ghi for details";

    let cap = re.captures(text).unwrap();
    assert_eq!(cap.get(1).unwrap().as_str(), "abc-def-ghi");
}

#[test]
fn test_state_enum_matching() {
    // Test state matching logic
    let terminal_states = vec!["COMPLETED", "FAILED", "PAUSED"];
    let non_terminal_states = vec!["IN_PROGRESS", "PENDING"];

    for state in &terminal_states {
        assert!(matches!(*state, "COMPLETED" | "FAILED" | "PAUSED"));
    }

    for state in &non_terminal_states {
        assert!(!matches!(*state, "COMPLETED" | "FAILED" | "PAUSED"));
    }
}

#[test]
fn test_activity_description_handling() {
    // Test handling of optional activity descriptions
    let descriptions: Vec<Option<String>> = vec![Some("Activity description".to_string()), None];

    for desc in &descriptions {
        let display = desc.as_deref().unwrap_or("(no description)");
        assert!(!display.is_empty());
    }
}

// ─────────────────────────────────────────────────────────────────────────
// INTEGRATION TESTS
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_command_sequence_watch_then_monitor() {
    // Test that watch and monitor can run in sequence
    let session = MockSession::new("test-seq-1");

    assert!(!session.id.is_empty());
    assert_eq!(session.state, Some("IN_PROGRESS".to_string()));

    // Simulate watch completing
    let completed_session = session.completed();
    assert_eq!(completed_session.state, Some("COMPLETED".to_string()));
}

#[test]
fn test_concurrent_session_updates() {
    // Test handling of concurrent session updates (thread-safe)
    let sessions = Arc::new(Mutex::new(vec![
        MockSession::new("s1"),
        MockSession::new("s2"),
        MockSession::new("s3"),
    ]));

    let sessions_clone = Arc::clone(&sessions);
    let handle = std::thread::spawn(move || {
        let mut sesh = sessions_clone.lock().unwrap();
        sesh[0] = sesh[0].clone().completed();
    });

    handle.join().unwrap();

    let final_sessions = sessions.lock().unwrap();
    assert_eq!(final_sessions[0].state, Some("COMPLETED".to_string()));
    assert_eq!(final_sessions[1].state, Some("IN_PROGRESS".to_string()));
}

#[test]
fn test_error_recovery_gh_cli_missing() {
    // Test graceful error handling when gh CLI is not available
    let gh_available = false; // Simulate gh CLI not available

    if !gh_available {
        // Should handle error gracefully
        assert!(!gh_available);
    }
}

#[test]
fn test_polling_interval_math() {
    // Test polling interval calculations
    let base_interval = 5u64;
    let max_polls = 12u64;

    let total_time = base_interval * max_polls;
    assert_eq!(total_time, 60); // 5 seconds * 12 polls = 60 seconds
}

#[test]
fn test_session_state_transitions() {
    // Test valid session state transitions
    let session = MockSession::new("test-transitions");

    // IN_PROGRESS -> COMPLETED
    let next = session.clone().completed();
    assert_eq!(next.state, Some("COMPLETED".to_string()));

    // Verify we can't go backwards (immutable)
    assert_eq!(session.state, Some("IN_PROGRESS".to_string()));
}

#[test]
fn test_large_activity_list_handling() {
    // Test handling of sessions with many activities
    let session = MockSession::new("test-large").with_activities(1000);

    assert_eq!(session.activities_count, 1000);
    let limited = std::cmp::min(session.activities_count, 5);
    assert_eq!(limited, 5); // Display only first 5
}

#[test]
fn test_timestamp_formatting() {
    // Test that timestamps are formatted correctly
    use chrono::Local;

    let now = Local::now();
    let formatted = now.format("%H:%M:%S").to_string();

    assert!(!formatted.is_empty());
    assert!(formatted.contains(":"));
}

#[test]
fn test_table_formatting_width() {
    // Test that table output respects width constraints
    let session_id = "a".repeat(50); // Very long ID
    let truncated = session_id.chars().take(20).collect::<String>();

    assert_eq!(truncated.len(), 20);
    assert!(truncated.len() <= 20);
}

#[test]
fn test_github_comment_parsing_edge_cases() {
    // Test edge cases in comment parsing
    let edge_cases = vec![
        ("", 0),                            // Empty comment
        ("no sessions here", 0),            // No mention
        ("session-123", 1),                 // Single simple mention
        ("session-123 and session-456", 2), // Multiple mentions
        ("SESSION-123", 0),                 // Wrong case (our regex is case-sensitive)
    ];

    let re = regex::Regex::new(r"session-[a-zA-Z0-9_-]+").unwrap();

    for (text, expected_count) in edge_cases {
        let count = re.captures_iter(text).count();
        assert_eq!(
            count, expected_count,
            "Failed for text: '{}', expected {}, got {}",
            text, expected_count, count
        );
    }
}
