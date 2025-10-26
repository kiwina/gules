//! Tests for jules-mcp tool argument structures.
//!
//! These tests verify that tool arguments can be properly serialized/deserialized
//! and have valid JSON schemas.

use jules_mcp::tools::*;
use schemars::schema_for;

#[test]
fn test_create_session_args_serialization() {
    let args = CreateSessionArgs {
        prompt: "Fix the bug".to_string(),
        source: "sources/github/owner/repo".to_string(),
        title: Some("Bug fix".to_string()),
        branch: "main".to_string(),
        automation_mode: Some("AUTO_CREATE_PR".to_string()),
    };

    let json = serde_json::to_string(&args).unwrap();
    assert!(json.contains("Fix the bug"));
    assert!(json.contains("sources/github/owner/repo"));

    let deserialized: CreateSessionArgs = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.prompt, "Fix the bug");
    assert_eq!(deserialized.source, "sources/github/owner/repo");
}

#[test]
fn test_create_session_args_defaults() {
    let json = r#"{"prompt":"test","source":"sources/github/test/test"}"#;
    let args: CreateSessionArgs = serde_json::from_str(json).unwrap();
    
    assert_eq!(args.prompt, "test");
    assert_eq!(args.branch, "main"); // default
    assert!(args.title.is_none());
    assert!(args.automation_mode.is_none());
}

#[test]
fn test_create_session_args_schema() {
    let schema = schema_for!(CreateSessionArgs);
    let schema_json = serde_json::to_string(&schema).unwrap();
    
    assert!(schema_json.contains("prompt"));
    assert!(schema_json.contains("source"));
    assert!(schema_json.contains("branch"));
}

#[test]
fn test_get_session_args() {
    let args = GetSessionArgs {
        session_id: "123456".to_string(),
    };

    let json = serde_json::to_string(&args).unwrap();
    let deserialized: GetSessionArgs = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.session_id, "123456");
}

#[test]
fn test_list_sessions_args_defaults() {
    let json = r#"{}"#;
    let args: ListSessionsArgs = serde_json::from_str(json).unwrap();
    
    assert_eq!(args.page_size, 10); // default
    assert!(args.page_token.is_none());
}

#[test]
fn test_send_message_args() {
    let args = SendMessageArgs {
        session_id: "123".to_string(),
        message: "Continue".to_string(),
    };

    let json = serde_json::to_string(&args).unwrap();
    assert!(json.contains("123"));
    assert!(json.contains("Continue"));
}

#[test]
fn test_approve_plan_args() {
    let args = ApprovePlanArgs {
        session_id: "456".to_string(),
    };

    let json = serde_json::to_string(&args).unwrap();
    let deserialized: ApprovePlanArgs = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.session_id, "456");
}

#[test]
fn test_list_sources_args_defaults() {
    let json = r#"{}"#;
    let args: ListSourcesArgs = serde_json::from_str(json).unwrap();
    
    assert_eq!(args.page_size, 30); // default
    assert!(args.filter.is_none());
    assert!(args.page_token.is_none());
}

#[test]
fn test_get_source_args() {
    let args = GetSourceArgs {
        source_id: "sources/github/owner/repo".to_string(),
    };

    let json = serde_json::to_string(&args).unwrap();
    assert!(json.contains("sources/github/owner/repo"));
}

#[test]
fn test_list_activities_args_defaults() {
    let json = r#"{"session_id":"789"}"#;
    let args: ListActivitiesArgs = serde_json::from_str(json).unwrap();
    
    assert_eq!(args.session_id, "789");
    assert_eq!(args.page_size, 30); // default
    assert!(args.page_token.is_none());
}

#[test]
fn test_get_activity_args() {
    let args = GetActivityArgs {
        session_id: "123".to_string(),
        activity_id: "abc".to_string(),
    };

    let json = serde_json::to_string(&args).unwrap();
    assert!(json.contains("123"));
    assert!(json.contains("abc"));
}

#[test]
fn test_all_args_have_schemas() {
    // Verify all argument types can generate JSON schemas
    let _ = schema_for!(CreateSessionArgs);
    let _ = schema_for!(GetSessionArgs);
    let _ = schema_for!(ListSessionsArgs);
    let _ = schema_for!(SendMessageArgs);
    let _ = schema_for!(ApprovePlanArgs);
    let _ = schema_for!(ListSourcesArgs);
    let _ = schema_for!(GetSourceArgs);
    let _ = schema_for!(ListActivitiesArgs);
    let _ = schema_for!(GetActivityArgs);
}
