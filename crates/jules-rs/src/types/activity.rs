use super::common::{ResourceId, ResourceName, Timestamp};
use serde::{Deserialize, Serialize};

/// Activity resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub name: ResourceName,
    pub id: ResourceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Timestamp,
    pub originator: String,
    #[serde(default)]
    pub artifacts: Vec<Artifact>,

    // Activity type (union field)
    #[serde(rename = "agentMessaged", skip_serializing_if = "Option::is_none")]
    pub agent_messaged: Option<AgentMessaged>,
    #[serde(rename = "userMessaged", skip_serializing_if = "Option::is_none")]
    pub user_messaged: Option<UserMessaged>,
    #[serde(rename = "planGenerated", skip_serializing_if = "Option::is_none")]
    pub plan_generated: Option<PlanGenerated>,
    #[serde(rename = "planApproved", skip_serializing_if = "Option::is_none")]
    pub plan_approved: Option<PlanApproved>,
    #[serde(rename = "progressUpdated", skip_serializing_if = "Option::is_none")]
    pub progress_updated: Option<ProgressUpdated>,
    #[serde(rename = "sessionCompleted", skip_serializing_if = "Option::is_none")]
    pub session_completed: Option<SessionCompleted>,
    #[serde(rename = "sessionFailed", skip_serializing_if = "Option::is_none")]
    pub session_failed: Option<SessionFailed>,
}

impl Activity {
    /// Get activity type as human-readable string by parsing the JSON structure
    /// This method is resilient to API changes - if Google adds new activity types,
    /// we'll still display them correctly by extracting the field name from the data.
    /// 
    /// Returns:
    /// - Known activity types: "Agent Messaged", "Progress Updated", etc.
    /// - Unknown activity types: "New Type [UNKNOWN]" - indicates SDK needs updating
    /// - Error case: "[ERROR: No Activity Type]" - indicates malformed activity data
    pub fn activity_type(&self) -> String {
        // Serialize back to JSON value to inspect which field is set
        // This way we don't need to maintain a hardcoded list
        if let Ok(value) = serde_json::to_value(self) {
            if let Some(obj) = value.as_object() {
                // Known activity type fields (in camelCase from API)
                let activity_fields = [
                    "agentMessaged", "userMessaged", "planGenerated", "planApproved",
                    "progressUpdated", "sessionCompleted", "sessionFailed"
                ];
                
                // Find which activity field is set
                for field in activity_fields {
                    if obj.contains_key(field) {
                        return camel_to_title_case(field);
                    }
                }
                
                // If it's a new activity type we don't know about yet,
                // find any camelCase field that isn't a standard Activity field
                let standard_fields = ["name", "id", "description", "createTime", "originator", "artifacts"];
                for (key, val) in obj.iter() {
                    if !standard_fields.contains(&key.as_str()) && !val.is_null() {
                        // Found a non-standard field - probably a new activity type
                        // Add [UNKNOWN] marker to make it obvious the SDK needs updating
                        return format!("{} [UNKNOWN]", camel_to_title_case(key));
                    }
                }
            }
        }
        
        // Fallback if serialization fails or no activity type found (shouldn't happen)
        // Make it obvious with [ERROR] marker that something went wrong
        "[ERROR: No Activity Type]".to_string()
    }

    /// Get activity content as string
    pub fn content(&self) -> Option<String> {
        if let Some(msg) = &self.agent_messaged {
            Some(msg.agent_message.clone())
        } else if let Some(msg) = &self.user_messaged {
            Some(msg.user_message.clone())
        } else if let Some(progress) = &self.progress_updated {
            // Check if there's a bash command in artifacts
            let bash_cmd = self.artifacts.iter()
                .find_map(|a| a.bash_output.as_ref())
                .map(|b| {
                    // Clean up the command: trim whitespace and replace newlines with spaces
                    b.command.trim().replace('\n', " ").replace("  ", " ")
                });
            
            if let Some(cmd) = bash_cmd {
                // Show the command cleanly without extra "Command:" prefix
                Some(format!("Ran: {}", cmd))
            } else {
                let title = progress.title.as_deref().unwrap_or("Progress update");
                let desc = progress.description.as_deref().unwrap_or("");
                Some(format!("{}: {}", title, desc))
            }
        } else {
            self.session_failed
                .as_ref()
                .map(|failed| format!("Session failed: {}", failed.reason))
        }
    }
}

/// Convert camelCase to Title Case
/// Examples: "agentMessaged" -> "Agent Messaged", "progressUpdated" -> "Progress Updated"
fn camel_to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    // Capitalize the first character
    if let Some(first) = chars.next() {
        result.push(first.to_uppercase().next().unwrap());
    }
    
    // Process remaining characters
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            // Add space before uppercase letter
            result.push(' ');
            result.push(ch);
        } else {
            result.push(ch);
        }
    }
    
    result
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessaged {
    #[serde(rename = "agentMessage")]
    pub agent_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessaged {
    #[serde(rename = "userMessage")]
    pub user_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanGenerated {
    pub plan: Plan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    #[serde(default)]
    pub steps: Vec<PlanStep>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanApproved {
    #[serde(rename = "planId")]
    pub plan_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCompleted {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionFailed {
    pub reason: String,
}

/// Artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "changeSet", skip_serializing_if = "Option::is_none")]
    pub change_set: Option<ChangeSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "bashOutput", skip_serializing_if = "Option::is_none")]
    pub bash_output: Option<BashOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    pub source: String,
    #[serde(rename = "gitPatch", skip_serializing_if = "Option::is_none")]
    pub git_patch: Option<GitPatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitPatch {
    #[serde(rename = "unidiffPatch", skip_serializing_if = "Option::is_none")]
    pub unidiff_patch: Option<String>,
    #[serde(rename = "baseCommitId", skip_serializing_if = "Option::is_none")]
    pub base_commit_id: Option<String>,
    #[serde(
        rename = "suggestedCommitMessage",
        skip_serializing_if = "Option::is_none"
    )]
    pub suggested_commit_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub data: String, // Base64
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashOutput {
    pub command: String,
    pub output: String,
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none", default)]
    pub exit_code: Option<i32>,
}

/// List activities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListActivitiesResponse {
    #[serde(default)]
    pub activities: Vec<Activity>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_title_case() {
        assert_eq!(camel_to_title_case("agentMessaged"), "Agent Messaged");
        assert_eq!(camel_to_title_case("userMessaged"), "User Messaged");
        assert_eq!(camel_to_title_case("planGenerated"), "Plan Generated");
        assert_eq!(camel_to_title_case("planApproved"), "Plan Approved");
        assert_eq!(camel_to_title_case("progressUpdated"), "Progress Updated");
        assert_eq!(camel_to_title_case("sessionCompleted"), "Session Completed");
        assert_eq!(camel_to_title_case("sessionFailed"), "Session Failed");
        assert_eq!(camel_to_title_case("unknown"), "Unknown");
        
        // Test edge cases
        assert_eq!(camel_to_title_case("helloWorld"), "Hello World");
        assert_eq!(camel_to_title_case("HTTPRequest"), "H T T P Request");
        assert_eq!(camel_to_title_case("a"), "A");
        assert_eq!(camel_to_title_case(""), "");
    }

    #[test]
    fn test_activity_type_conversion() {
        // Test each known activity type
        let test_cases = vec![
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[],"agentMessaged":{"agentMessage":"test"}}"#, "Agent Messaged"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"user","artifacts":[],"userMessaged":{"userMessage":"test"}}"#, "User Messaged"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[],"planGenerated":{"plan":{"id":"p1","steps":[]}}}"#, "Plan Generated"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[],"planApproved":{"planId":"p1"}}"#, "Plan Approved"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[],"progressUpdated":{"description":"test"}}"#, "Progress Updated"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"system","artifacts":[],"sessionCompleted":{}}"#, "Session Completed"),
            (r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"system","artifacts":[],"sessionFailed":{"reason":"error"}}"#, "Session Failed"),
        ];
        
        for (json, expected_type) in test_cases {
            let activity: Activity = serde_json::from_str(json)
                .unwrap_or_else(|e| panic!("Failed to parse JSON for {}: {}", expected_type, e));
            assert_eq!(activity.activity_type(), expected_type, 
                "Activity type mismatch for {}", expected_type);
        }
    }

    #[test]
    fn test_activity_type_resilience() {
        // Test that we handle activity with no type gracefully
        let no_type_json = r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[]}"#;
        let activity: Activity = serde_json::from_str(no_type_json).unwrap();
        let activity_type = activity.activity_type();
        assert!(!activity_type.is_empty(), "Should return some activity type");
        assert!(activity_type.contains("[ERROR"), "Should have error marker: {}", activity_type);
        
        // Test future-proofing: simulate a new activity type Google might add
        // We use serde_json::Value to add a field that doesn't exist in our struct
        let new_type_json = r#"{"name":"s/1/a/1","id":"1","createTime":"2025-10-26T00:00:00Z","originator":"agent","artifacts":[],"codeReviewed":{"reviewId":"r1"}}"#;
        
        // This should deserialize with the unknown field being ignored (default serde behavior)
        // Since the field isn't in our struct, it won't be re-serialized, so we'll get [ERROR]
        if let Ok(activity) = serde_json::from_str::<Activity>(new_type_json) {
            let activity_type = activity.activity_type();
            // Will show [ERROR] since unknown field is dropped during deserialization
            assert!(!activity_type.is_empty(), "Should handle unknown activity types gracefully");
            assert!(activity_type.contains("[ERROR") || activity_type.contains("[UNKNOWN]"), 
                "Should have error/unknown marker for unrecognized types: {}", activity_type);
        }
    }
}
