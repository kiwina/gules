//! Test filtering with real activity data from activities.json
//!
//! This test reads the actual activities.json sample file and verifies
//! that our filtering logic works correctly with real data.

use gules::commands::filter_activities::ActivityTypeFilter;
use jules_rs::types::activity::*;

#[test]
fn test_filter_real_activities_json() {
    // Read the activities.json file
    let json_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("activities.json");

    if !json_path.exists() {
        eprintln!(
            "Skipping test - activities.json not found at: {:?}",
            json_path
        );
        return;
    }

    let json_content = std::fs::read_to_string(&json_path).expect("Failed to read activities.json");

    // Parse activities array directly (format from gules activities command)
    let activities: Vec<Activity> =
        serde_json::from_str(&json_content).expect("Failed to parse activities array");

    // Count total activities
    println!("Total activities: {}", activities.len());
    assert!(activities.len() > 0, "Expected activities in sample file");
    println!("Found {} activities in sample file", activities.len());

    // Count by type
    let mut counts = std::collections::HashMap::new();
    for activity in &activities {
        let activity_type = if activity.agent_messaged.is_some() {
            "agent_messaged"
        } else if activity.user_messaged.is_some() {
            "user_messaged"
        } else if activity.plan_generated.is_some() {
            "plan_generated"
        } else if activity.plan_approved.is_some() {
            "plan_approved"
        } else if activity.progress_updated.is_some() {
            "progress_updated"
        } else if activity.session_completed.is_some() {
            "session_completed"
        } else if activity.session_failed.is_some() {
            "session_failed"
        } else {
            "unknown"
        };

        *counts.entry(activity_type).or_insert(0) += 1;
    }

    println!("Activity counts:");
    for (activity_type, count) in &counts {
        println!("  {}: {}", activity_type, count);
    }

    // Verify we have some of each main type
    assert!(
        counts.get("progress_updated").copied().unwrap_or(0) > 0,
        "Expected some progress_updated activities"
    );
    assert!(
        counts.get("plan_generated").copied().unwrap_or(0) > 0,
        "Expected some plan_generated activities"
    );
    assert!(
        counts.get("plan_approved").copied().unwrap_or(0) > 0,
        "Expected some plan_approved activity"
    );
    assert!(
        counts.get("agent_messaged").copied().unwrap_or(0) > 0,
        "Expected some agent_messaged activity"
    );
    assert!(
        counts.get("user_messaged").copied().unwrap_or(0) > 0,
        "Expected some user_messaged activity"
    );

    // Test filtering by Progress type
    let progress_filter = ActivityTypeFilter::Progress;
    let filtered_progress: Vec<_> = activities
        .iter()
        .filter(|a| progress_filter.matches(a))
        .collect();

    println!("\nProgress activities: {}", filtered_progress.len());
    assert!(
        filtered_progress.len() > 0,
        "Expected some progress activities"
    );

    // Test filtering for activities with bash output
    let with_bash: Vec<_> = activities
        .iter()
        .filter(|a| {
            a.artifacts
                .iter()
                .any(|artifact| artifact.bash_output.is_some())
        })
        .collect();

    println!("Activities with bash output: {}", with_bash.len());

    // Count how many actually have bash output
    for activity in &with_bash {
        println!("  Activity {} has bash output:", activity.id);
        for artifact in &activity.artifacts {
            if let Some(bash) = &artifact.bash_output {
                let cmd = bash.command.as_deref().unwrap_or("");
                println!("    Command: {}", cmd.lines().next().unwrap_or(""));
                println!("    Exit code: {:?}", bash.exit_code);
            }
        }
    }

    // At least the first activity should have bash output
    assert!(
        with_bash.len() >= 1,
        "Expected at least 1 activity with bash output"
    );

    // Test filtering for errors (bash_output with exit_code != 0)
    let errors: Vec<_> = activities
        .iter()
        .filter(|a| {
            a.artifacts.iter().any(|artifact| {
                artifact
                    .bash_output
                    .as_ref()
                    .and_then(|b| b.exit_code)
                    .map(|code| code != 0)
                    .unwrap_or(false)
            })
        })
        .collect();

    println!(
        "\nActivities with errors (exit_code != 0): {}",
        errors.len()
    );

    // Count errors
    for activity in &errors {
        for artifact in &activity.artifacts {
            if let Some(bash) = &artifact.bash_output {
                if let Some(exit_code) = bash.exit_code {
                    if exit_code != 0 {
                        println!("  Activity {} - exit code: {}", activity.id, exit_code);
                    }
                }
            }
        }
    }

    // Test combined filter: Progress + bash output
    let progress_with_bash: Vec<_> = activities
        .iter()
        .filter(|a| progress_filter.matches(a))
        .filter(|a| {
            a.artifacts
                .iter()
                .any(|artifact| artifact.bash_output.is_some())
        })
        .collect();

    println!(
        "\nProgress activities with bash output: {}",
        progress_with_bash.len()
    );
    assert!(
        progress_with_bash.len() >= 1,
        "Expected at least 1 progress activity with bash output"
    );
}

#[test]
fn test_detect_bash_output_in_progress_updates() {
    // Read the activities.json file
    let json_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("activities.json");

    if !json_path.exists() {
        eprintln!("Skipping test - activities.json not found");
        return;
    }

    let json_content = std::fs::read_to_string(&json_path).unwrap();
    // Parse activities array directly (format from gules activities command)
    let activities: Vec<Activity> = serde_json::from_str(&json_content).unwrap();

    // Find any activity that has bash output (since the specific ID may not be in this session)
    let bash_activity = activities
        .iter()
        .find(|a| {
            a.artifacts
                .iter()
                .any(|artifact| artifact.bash_output.is_some())
        })
        .expect("Could not find any activity with bash output in sample data");

    println!("Found activity: {}", bash_activity.id);
    println!(
        "  progress_updated: {}",
        bash_activity.progress_updated.is_some()
    );
    println!("  artifacts count: {}", bash_activity.artifacts.len());

    for (i, artifact) in bash_activity.artifacts.iter().enumerate() {
        println!(
            "  artifact {}: bash_output present: {}",
            i,
            artifact.bash_output.is_some()
        );
        if let Some(bash) = &artifact.bash_output {
            let cmd = bash.command.as_deref().unwrap_or("");
            println!("    command: {}", cmd.lines().next().unwrap_or(""));
            println!("    exit_code: {:?}", bash.exit_code);
            let out = bash.output.as_deref().unwrap_or("");
            println!("    output: {}", out.chars().take(50).collect::<String>());
        }
    }

    // Verify it's a progress_updated activity
    assert!(
        bash_activity.progress_updated.is_some(),
        "Should be a progress_updated activity"
    );

    // Verify it has bash output in artifacts
    assert!(
        bash_activity
            .artifacts
            .iter()
            .any(|a| a.bash_output.is_some()),
        "Should have bash output in artifacts"
    );

    // Verify the Progress filter matches it
    let progress_filter = ActivityTypeFilter::Progress;
    assert!(
        progress_filter.matches(bash_activity),
        "Progress filter should match"
    );
}
