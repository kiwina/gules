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
    
    // Parse the outer wrapper
    let response: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    // Extract the activities array
    let activities_json = &response["activities"];
    assert!(activities_json.is_array(), "Expected 'activities' to be an array");
    
    // Try to deserialize each activity individually to catch specific errors
    let activities_array = activities_json.as_array().unwrap();
    println!("Testing {} activities...", activities_array.len());
    
    for (i, activity_json) in activities_array.iter().enumerate() {
        let activity: Activity = serde_json::from_value(activity_json.clone())
            .unwrap_or_else(|e| {
                eprintln!("\nFailed to parse activity {}: {}", i, e);
                eprintln!("Activity JSON: {}", serde_json::to_string_pretty(activity_json).unwrap());
                panic!("Deserialization failed for activity {}", i);
            });
        
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
                assert!(!bash.command.is_empty(), 
                    "Activity {} artifact {} has empty bash command", i, j);
                assert!(!bash.output.is_empty(), 
                    "Activity {} artifact {} has empty bash output", i, j);
                println!("  └─ Artifact {}: bash output (exit: {:?})", j, bash.exit_code);
            }
            
            if let Some(changeset) = &artifact.change_set {
                assert!(!changeset.source.is_empty(),
                    "Activity {} artifact {} has empty source", i, j);
                if let Some(patch) = &changeset.git_patch {
                    assert!(!patch.unidiff_patch.is_empty(),
                        "Activity {} artifact {} has empty patch", i, j);
                    assert!(!patch.base_commit_id.is_empty(),
                        "Activity {} artifact {} has empty commit id", i, j);
                    println!("  └─ Artifact {}: git patch ({} lines)", 
                        j, patch.unidiff_patch.lines().count());
                }
            }
        }
    }
    
    // Now deserialize all at once
    let all_activities: Vec<Activity> = serde_json::from_value(activities_json.clone())
        .expect("Failed to deserialize full activities array");
    
    assert_eq!(all_activities.len(), activities_array.len(),
        "Deserialized count doesn't match JSON count");
    
    println!("\n✅ Successfully deserialized all {} activities", all_activities.len());
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
    
    // Try to deserialize as ListActivitiesResponse
    let response: ListActivitiesResponse = serde_json::from_str(&json_content)
        .expect("Failed to deserialize as ListActivitiesResponse");
    
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
    
    assert_eq!(bash.command, "\ncd /app\necho do setup\n");
    assert_eq!(bash.output, "do setup");
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
    
    assert_eq!(bash.command, "\nmake dev");
    assert_eq!(bash.exit_code, Some(2));
    assert!(bash.output.starts_with("cargo fmt"));
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
    let response: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    let activities: Vec<Activity> = serde_json::from_value(response["activities"].clone()).unwrap();
    
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
