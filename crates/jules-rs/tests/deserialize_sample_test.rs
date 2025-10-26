//! Test deserialization of actual activities.json sample file
//!
//! This validates that jules-rs types correctly parse real API responses.

use jules_rs::types::activity::*;
use std::path::Path;

#[test]
fn test_deserialize_activities_json() {
    // Find activities.json in the workspace root
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("activities.json");
    
    if !json_path.exists() {
        eprintln!("Skipping test - activities.json not found at: {:?}", json_path);
        return;
    }

    let json_content = std::fs::read_to_string(&json_path)
        .expect("Failed to read activities.json");
    
    // Parse activities array directly (the JSON is an array, not wrapped)
    let activities: Vec<Activity> = serde_json::from_str(&json_content)
        .expect("Failed to parse activities array");
    
    println!("Testing {} activities...", activities.len());
    
    // Verify we can deserialize all activities
    for (i, activity) in activities.iter().enumerate() {
        // Validate basic fields
        assert!(!activity.id.is_empty(), "Activity {} has empty id", i);
        assert!(!activity.name.is_empty(), "Activity {} has empty name", i);
        assert!(!activity.create_time.is_empty(), "Activity {} has empty create_time", i);
        
        // Check activity type is set
        let activity_type = activity.activity_type();
        assert_ne!(activity_type, "Unknown", "Activity {} has unknown type", i);
        
        println!("✓ Activity {} ({}) - {}", i, activity.id, activity_type);
        
        // Validate artifacts
        for (j, artifact) in activity.artifacts.iter().enumerate() {
            if let Some(bash) = &artifact.bash_output {
                if let Some(cmd) = &bash.command {
                    assert!(!cmd.is_empty(), 
                        "Activity {} artifact {} has empty bash command", i, j);
                }
                if let Some(output) = &bash.output {
                    assert!(!output.is_empty(), 
                        "Activity {} artifact {} has empty bash output", i, j);
                }
                println!("  └─ Artifact {}: bash output (exit: {:?})", j, bash.exit_code);
            }
            
            if let Some(changeset) = &artifact.change_set {
                assert!(!changeset.source.is_empty(),
                    "Activity {} artifact {} has empty source", i, j);
                if let Some(patch) = &changeset.git_patch {
                    if let Some(unidiff) = &patch.unidiff_patch {
                        assert!(!unidiff.is_empty(),
                            "Activity {} artifact {} has empty patch", i, j);
                        println!("  └─ Artifact {}: git patch ({} lines)", 
                            j, unidiff.lines().count());
                    } else {
                        println!("  └─ Artifact {}: git patch (no diff)", j);
                    }
                    if let Some(base_commit) = &patch.base_commit_id {
                        assert!(!base_commit.is_empty(),
                            "Activity {} artifact {} has empty commit id", i, j);
                    }
                }
            }
        }
    }
    
    println!("\n✅ Successfully deserialized all {} activities", activities.len());
}

#[test]
fn test_deserialize_list_activities_response() {
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("activities.json");
    
    if !json_path.exists() {
        eprintln!("Skipping test - activities.json not found");
        return;
    }

    let json_content = std::fs::read_to_string(&json_path)
        .expect("Failed to read activities.json");
    
    // The JSON is a plain activities array, wrap it to create ListActivitiesResponse
    let activities: Vec<Activity> = serde_json::from_str(&json_content)
        .expect("Failed to parse activities array");
    
    // Create a proper ListActivitiesResponse
    let response = ListActivitiesResponse {
        activities,
        next_page_token: None,
    };
    
    println!("Activities: {}", response.activities.len());
    println!("Next page token: {:?}", response.next_page_token);
    
    assert!(!response.activities.is_empty(), "Expected some activities");
}

#[test]
fn test_bash_output_with_missing_exit_code() {
    // The first activity in the sample has bash output without exitCode
    let json = r#"{
        "command": "\ncd /app\necho do setup\n",
        "output": "do setup"
    }"#;
    
    let bash: BashOutput = serde_json::from_str(json)
        .expect("Failed to deserialize BashOutput without exitCode");
    
    assert_eq!(bash.command, Some("\ncd /app\necho do setup\n".to_string()));
    assert_eq!(bash.output, Some("do setup".to_string()));
    assert_eq!(bash.exit_code, None, "Exit code should be None when missing");
}

#[test]
fn test_bash_output_with_exit_code() {
    let json = r#"{
        "command": "\nmake dev",
        "exitCode": 2,
        "output": "cargo fmt --all\ncargo clippy..."
    }"#;
    
    let bash: BashOutput = serde_json::from_str(json)
        .expect("Failed to deserialize BashOutput with exitCode");
    
    assert_eq!(bash.command, Some("\nmake dev".to_string()));
    assert_eq!(bash.exit_code, Some(2));
    assert!(bash.output.as_ref().unwrap().starts_with("cargo fmt"));
}

#[test]
fn test_activity_content_extraction() {
    let json_path = Path::new(env!("CARGO_MANIFEST_DIR"))
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
    // Parse activities array directly
    let activities: Vec<Activity> = serde_json::from_str(&json_content).unwrap();
    
    let mut content_count = 0;
    for activity in &activities {
        if let Some(content) = activity.content() {
            content_count += 1;
            assert!(!content.is_empty(), "Content should not be empty");
        }
    }
    
    println!("Activities with extractable content: {}/{}", content_count, activities.len());
    assert!(content_count > 0, "Expected some activities to have content");
}
