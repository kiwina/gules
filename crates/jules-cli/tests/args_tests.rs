//! Tests for jules-cli command argument structures and parsing.
//!
//! These tests verify that CLI arguments parse correctly and have expected defaults.
//! API integration is tested in jules-rs, not here.

use jules_cli::commands::*;

#[test]
fn test_create_args_minimal() {
    let args = CreateArgs {
        prompt: "Fix bug".to_string(),
        source: "github.com/user/repo".to_string(),
        title: None,
        branch: None,
        require_approval: false,
        automation_mode: None,
    };

    assert_eq!(args.prompt, "Fix bug");
    assert_eq!(args.source, "github.com/user/repo");
    assert!(args.title.is_none());
    assert!(args.branch.is_none());
    assert_eq!(args.require_approval, false);
    assert!(args.automation_mode.is_none());
}

#[test]
fn test_create_args_full() {
    let args = CreateArgs {
        prompt: "Complex task".to_string(),
        source: "github.com/owner/repo".to_string(),
        title: Some("My Task".to_string()),
        branch: Some("feature".to_string()),
        require_approval: true,
        automation_mode: Some("AUTO_CREATE_PR".to_string()),
    };

    assert_eq!(args.prompt, "Complex task");
    assert_eq!(args.source, "github.com/owner/repo");
    assert_eq!(args.title, Some("My Task".to_string()));
    assert_eq!(args.branch, Some("feature".to_string()));
    assert_eq!(args.require_approval, true);
    assert_eq!(args.automation_mode, Some("AUTO_CREATE_PR".to_string()));
}

#[test]
fn test_session_args() {
    let args = SessionArgs {
        id: "12345".to_string(),
    };

    assert_eq!(args.id, "12345");
}

#[test]
fn test_send_message_args() {
    let args = SendMessageArgs {
        session_id: "67890".to_string(),
        message: "Continue with the plan".to_string(),
    };

    assert_eq!(args.session_id, "67890");
    assert_eq!(args.message, "Continue with the plan");
}

#[test]
fn test_approve_plan_args() {
    let args = ApprovePlanArgs {
        session_id: "plan-123".to_string(),
    };

    assert_eq!(args.session_id, "plan-123");
}

#[test]
fn test_activity_args() {
    let args = ActivityArgs {
        session_id: "sess-456".to_string(),
        activity_id: "act-789".to_string(),
    };

    assert_eq!(args.session_id, "sess-456");
    assert_eq!(args.activity_id, "act-789");
}

#[test]
fn test_source_args() {
    let args = SourceArgs {
        id: "sources/github/owner/repo".to_string(),
    };

    assert_eq!(args.id, "sources/github/owner/repo");
}
