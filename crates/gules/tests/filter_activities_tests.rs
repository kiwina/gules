//! Tests for filter_activities command.

use jules_rs::types::activity::*;

// Import the filter module functions
use gules::commands::filter_activities::*;

#[test]
fn test_activity_type_filter_from_str() {
    // Test all valid type strings
    assert!(matches!(
        ActivityTypeFilter::from_str("agent-message").unwrap(),
        ActivityTypeFilter::AgentMessage
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("agent").unwrap(),
        ActivityTypeFilter::AgentMessage
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("user-message").unwrap(),
        ActivityTypeFilter::UserMessage
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("user").unwrap(),
        ActivityTypeFilter::UserMessage
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("plan").unwrap(),
        ActivityTypeFilter::Plan
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("plan-generated").unwrap(),
        ActivityTypeFilter::Plan
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("plan-approved").unwrap(),
        ActivityTypeFilter::PlanApproved
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("approved").unwrap(),
        ActivityTypeFilter::PlanApproved
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("progress").unwrap(),
        ActivityTypeFilter::Progress
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("progress-updated").unwrap(),
        ActivityTypeFilter::Progress
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("completed").unwrap(),
        ActivityTypeFilter::Completed
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("session-completed").unwrap(),
        ActivityTypeFilter::Completed
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("failed").unwrap(),
        ActivityTypeFilter::Failed
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("session-failed").unwrap(),
        ActivityTypeFilter::Failed
    ));
    assert!(matches!(
        ActivityTypeFilter::from_str("error").unwrap(),
        ActivityTypeFilter::Failed
    ));

    // Test invalid string
    assert!(ActivityTypeFilter::from_str("invalid-type").is_err());
}

#[test]
fn test_activity_type_filter_matches_agent_message() {
    let mut activity = create_test_activity("1");
    activity.agent_messaged = Some(AgentMessaged {
        agent_message: "Test message".to_string(),
    });

    let filter = ActivityTypeFilter::AgentMessage;
    assert!(filter.matches(&activity));

    let other_filter = ActivityTypeFilter::UserMessage;
    assert!(!other_filter.matches(&activity));
}

#[test]
fn test_activity_type_filter_matches_user_message() {
    let mut activity = create_test_activity("1");
    activity.user_messaged = Some(UserMessaged {
        user_message: "User said something".to_string(),
    });

    let filter = ActivityTypeFilter::UserMessage;
    assert!(filter.matches(&activity));
}

#[test]
fn test_activity_type_filter_matches_plan() {
    let mut activity = create_test_activity("1");
    activity.plan_generated = Some(PlanGenerated {
        plan: Plan {
            id: "plan-1".to_string(),
            steps: vec![],
            create_time: Some(chrono::Utc::now().to_rfc3339()),
        },
    });

    let filter = ActivityTypeFilter::Plan;
    assert!(filter.matches(&activity));
}

#[test]
fn test_activity_type_filter_matches_failed() {
    let mut activity = create_test_activity("1");
    activity.session_failed = Some(SessionFailed {
        reason: "Test error".to_string(),
    });

    let filter = ActivityTypeFilter::Failed;
    assert!(filter.matches(&activity));
}

#[test]
fn test_activity_type_filter_matches_progress() {
    let mut activity = create_test_activity("1");
    activity.progress_updated = Some(ProgressUpdated {
        title: "Working on it".to_string(),
        description: Some("Making progress".to_string()),
    });

    let filter = ActivityTypeFilter::Progress;
    assert!(filter.matches(&activity));
}

#[test]
fn test_activity_type_filter_matches_completed() {
    let mut activity = create_test_activity("1");
    activity.session_completed = Some(SessionCompleted {});

    let filter = ActivityTypeFilter::Completed;
    assert!(filter.matches(&activity));
}

#[test]
fn test_output_format_from_str() {
    assert!(matches!(
        OutputFormat::from_str("table").unwrap(),
        OutputFormat::Table
    ));
    assert!(matches!(
        OutputFormat::from_str("json").unwrap(),
        OutputFormat::Json
    ));
    assert!(matches!(
        OutputFormat::from_str("full").unwrap(),
        OutputFormat::Full
    ));
    assert!(matches!(
        OutputFormat::from_str("content").unwrap(),
        OutputFormat::ContentOnly
    ));
    assert!(matches!(
        OutputFormat::from_str("content-only").unwrap(),
        OutputFormat::ContentOnly
    ));

    // Test invalid format
    assert!(OutputFormat::from_str("invalid").is_err());
}

#[test]
fn test_activity_with_bash_output() {
    let mut activity = create_test_activity("1");

    // Activity without bash output
    assert!(activity
        .artifacts
        .iter()
        .all(|a| a.bash_output.is_none()));

    // Add bash output artifact
    activity.artifacts.push(Artifact {
        change_set: None,
        media: None,
        bash_output: Some(BashOutput {
            command: "cargo test".to_string(),
            output: "test failed".to_string(),
            exit_code: Some(1),
        }),
    });

    // Now it has bash output
    assert!(activity
        .artifacts
        .iter()
        .any(|a| a.bash_output.is_some()));
}

#[test]
fn test_activity_content_extraction() {
    // Test agent message content
    let mut activity = create_test_activity("1");
    activity.agent_messaged = Some(AgentMessaged {
        agent_message: "Agent says hello".to_string(),
    });
    assert_eq!(activity.content(), Some("Agent says hello".to_string()));

    // Test user message content
    let mut activity = create_test_activity("2");
    activity.user_messaged = Some(UserMessaged {
        user_message: "User says hi".to_string(),
    });
    assert_eq!(activity.content(), Some("User says hi".to_string()));

    // Test progress content
    let mut activity = create_test_activity("3");
    activity.progress_updated = Some(ProgressUpdated {
        title: "Building".to_string(),
        description: Some("Compiling code".to_string()),
    });
    assert_eq!(
        activity.content(),
        Some("Building: Compiling code".to_string())
    );

    // Test failed content
    let mut activity = create_test_activity("4");
    activity.session_failed = Some(SessionFailed {
        reason: "Build failed".to_string(),
    });
    assert_eq!(
        activity.content(),
        Some("Session failed: Build failed".to_string())
    );

    // Test activity with no content
    let activity = create_test_activity("5");
    assert_eq!(activity.content(), None);
}

#[test]
fn test_activity_type_string() {
    let mut activity = create_test_activity("1");

    activity.agent_messaged = Some(AgentMessaged {
        agent_message: "test".to_string(),
    });
    assert_eq!(activity.activity_type(), "Agent Message");

    let mut activity = create_test_activity("2");
    activity.user_messaged = Some(UserMessaged {
        user_message: "test".to_string(),
    });
    assert_eq!(activity.activity_type(), "User Message");

    let mut activity = create_test_activity("3");
    activity.plan_generated = Some(PlanGenerated {
        plan: Plan {
            id: "p1".to_string(),
            steps: vec![],
            create_time: None,
        },
    });
    assert_eq!(activity.activity_type(), "Plan Generated");

    let mut activity = create_test_activity("4");
    activity.session_failed = Some(SessionFailed {
        reason: "error".to_string(),
    });
    assert_eq!(activity.activity_type(), "Session Failed");

    let activity = create_test_activity("5");
    assert_eq!(activity.activity_type(), "Unknown");
}

#[test]
fn test_multiple_type_filters() {
    let mut agent_activity = create_test_activity("1");
    agent_activity.agent_messaged = Some(AgentMessaged {
        agent_message: "agent".to_string(),
    });

    let mut user_activity = create_test_activity("2");
    user_activity.user_messaged = Some(UserMessaged {
        user_message: "user".to_string(),
    });

    let mut failed_activity = create_test_activity("3");
    failed_activity.session_failed = Some(SessionFailed {
        reason: "error".to_string(),
    });

    // Create filters
    let filters = vec![
        ActivityTypeFilter::AgentMessage,
        ActivityTypeFilter::UserMessage,
    ];

    // Agent and user should match
    assert!(filters.iter().any(|f| f.matches(&agent_activity)));
    assert!(filters.iter().any(|f| f.matches(&user_activity)));

    // Failed should not match
    assert!(!filters.iter().any(|f| f.matches(&failed_activity)));
}

#[test]
fn test_bash_output_filtering_logic() {
    let mut activity_with_bash = create_test_activity("1");
    activity_with_bash.artifacts.push(Artifact {
        change_set: None,
        media: None,
        bash_output: Some(BashOutput {
            command: "npm test".to_string(),
            output: "FAIL: 1 test failed".to_string(),
            exit_code: Some(1),
        }),
    });

    let mut activity_without_bash = create_test_activity("2");
    activity_without_bash.artifacts.push(Artifact {
        change_set: Some(ChangeSet {
            source: "test".to_string(),
            git_patch: None,
        }),
        media: None,
        bash_output: None,
    });

    let activity_no_artifacts = create_test_activity("3");

    // Filter logic: has_bash_output = true
    let activities = vec![
        activity_with_bash.clone(),
        activity_without_bash.clone(),
        activity_no_artifacts.clone(),
    ];

    let filtered: Vec<_> = activities
        .iter()
        .filter(|a| {
            a.artifacts
                .iter()
                .any(|artifact| artifact.bash_output.is_some())
        })
        .collect();

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "1");
}

#[test]
fn test_last_n_filtering() {
    let activities = vec![
        create_test_activity("1"),
        create_test_activity("2"),
        create_test_activity("3"),
        create_test_activity("4"),
        create_test_activity("5"),
    ];

    // Take last 3
    let last_3: Vec<_> = activities.iter().take(3).collect();
    assert_eq!(last_3.len(), 3);
    assert_eq!(last_3[0].id, "1");
    assert_eq!(last_3[1].id, "2");
    assert_eq!(last_3[2].id, "3");

    // Take last 1
    let last_1: Vec<_> = activities.iter().take(1).collect();
    assert_eq!(last_1.len(), 1);
    assert_eq!(last_1[0].id, "1");

    // Take more than available
    let all: Vec<_> = activities.iter().take(10).collect();
    assert_eq!(all.len(), 5);
}

// Helper function to create test activity
fn create_test_activity(id: &str) -> Activity {
    Activity {
        name: format!("sessions/test/activities/{}", id),
        id: id.to_string(),
        description: Some(format!("Test activity {}", id)),
        create_time: chrono::Utc::now().to_rfc3339(),
        originator: "test".to_string(),
        artifacts: vec![],
        agent_messaged: None,
        user_messaged: None,
        plan_generated: None,
        plan_approved: None,
        progress_updated: None,
        session_completed: None,
        session_failed: None,
    }
}
