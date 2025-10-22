//! Comprehensive validation test for activities.json deserialization
//! This ensures jules-rs types correctly parse real API responses

use jules_rs::types::activity::*;

#[test]
fn quick_validate_activities_json() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("activities.json");
    let json = std::fs::read_to_string(path);
    
    if json.is_err() {
        println!("⚠️  Skipping - activities.json not found");
        return;
    }
    
    let json = json.unwrap();
    
    // Try to deserialize as ListActivitiesResponse
    match serde_json::from_str::<ListActivitiesResponse>(&json) {
        Ok(response) => {
            println!("✅ Deserialized ListActivitiesResponse");
            println!("   Activities: {}", response.activities.len());
            println!("   Next page token: {}", 
                response.next_page_token.as_deref().unwrap_or("none"));
            
            let mut stats = ActivityStats::default();
            
            // Validate each activity
            for (i, activity) in response.activities.iter().enumerate() {
                assert!(!activity.id.is_empty(), "Activity {} has empty id", i);
                assert!(!activity.name.is_empty(), "Activity {} has empty name", i);
                assert!(!activity.create_time.is_empty(), "Activity {} has empty create_time", i);
                assert!(!activity.originator.is_empty(), "Activity {} has empty originator", i);
                
                // Count activity types
                if activity.agent_messaged.is_some() { stats.agent_messaged += 1; }
                if activity.user_messaged.is_some() { stats.user_messaged += 1; }
                if activity.plan_generated.is_some() { stats.plan_generated += 1; }
                if activity.plan_approved.is_some() { stats.plan_approved += 1; }
                if activity.progress_updated.is_some() { stats.progress_updated += 1; }
                if activity.session_completed.is_some() { stats.session_completed += 1; }
                if activity.session_failed.is_some() { stats.session_failed += 1; }
                
                // Validate artifacts
                for (j, artifact) in activity.artifacts.iter().enumerate() {
                    if let Some(bash) = &artifact.bash_output {
                        stats.bash_outputs += 1;
                        assert!(!bash.command.is_empty(), 
                            "Activity {} artifact {} has empty bash command", i, j);
                        assert!(!bash.output.is_empty(),
                            "Activity {} artifact {} has empty bash output", i, j);
                        if bash.exit_code.is_none() {
                            stats.bash_without_exitcode += 1;
                        }
                    }
                    if let Some(cs) = &artifact.change_set {
                        stats.changesets += 1;
                        assert!(!cs.source.is_empty(),
                            "Activity {} artifact {} has empty source", i, j);
                        if let Some(patch) = &cs.git_patch {
                            stats.git_patches += 1;
                            assert!(!patch.unidiff_patch.is_empty(),
                                "Activity {} artifact {} has empty patch", i, j);
                            assert!(!patch.base_commit_id.is_empty(),
                                "Activity {} artifact {} has empty commit id", i, j);
                            if patch.suggested_commit_message.is_none() {
                                stats.patches_without_suggestion += 1;
                            }
                        }
                    }
                    if artifact.media.is_some() {
                        stats.media_artifacts += 1;
                    }
                }
            }
            
            println!("\n📊 Activity Statistics:");
            println!("   agent_messaged: {}", stats.agent_messaged);
            println!("   user_messaged: {}", stats.user_messaged);
            println!("   plan_generated: {}", stats.plan_generated);
            println!("   plan_approved: {}", stats.plan_approved);
            println!("   progress_updated: {}", stats.progress_updated);
            println!("   session_completed: {}", stats.session_completed);
            println!("   session_failed: {}", stats.session_failed);
            
            println!("\n📦 Artifact Statistics:");
            println!("   bash outputs: {} ({} without exitCode)", 
                stats.bash_outputs, stats.bash_without_exitcode);
            println!("   changesets: {}", stats.changesets);
            println!("   git patches: {} ({} without suggested message)", 
                stats.git_patches, stats.patches_without_suggestion);
            println!("   media artifacts: {}", stats.media_artifacts);
            
            // Verify expected counts from analysis
            assert_eq!(response.activities.len(), 30, "Expected 30 activities");
            assert_eq!(stats.bash_outputs, 17, "Expected 17 bash outputs");
            assert_eq!(stats.bash_without_exitcode, 1, "Expected 1 bash without exitCode");
            assert_eq!(stats.git_patches, 8, "Expected 8 git patches");
            assert_eq!(stats.patches_without_suggestion, 8, "Expected all patches without suggestion");
            
            println!("\n✅ All validations passed!");
        }
        Err(e) => {
            eprintln!("❌ Failed to parse activities.json:");
            eprintln!("   {}", e);
            panic!("Deserialization failed - check the error above for details");
        }
    }
}

#[derive(Default)]
struct ActivityStats {
    agent_messaged: usize,
    user_messaged: usize,
    plan_generated: usize,
    plan_approved: usize,
    progress_updated: usize,
    session_completed: usize,
    session_failed: usize,
    bash_outputs: usize,
    bash_without_exitcode: usize,
    changesets: usize,
    git_patches: usize,
    patches_without_suggestion: usize,
    media_artifacts: usize,
}
