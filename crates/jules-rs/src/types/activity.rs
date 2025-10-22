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
    /// Get activity type as string
    pub fn activity_type(&self) -> &'static str {
        if self.agent_messaged.is_some() {
            return "Agent Message";
        }
        if self.user_messaged.is_some() {
            return "User Message";
        }
        if self.plan_generated.is_some() {
            return "Plan Generated";
        }
        if self.plan_approved.is_some() {
            return "Plan Approved";
        }
        if self.progress_updated.is_some() {
            return "Progress Update";
        }
        if self.session_completed.is_some() {
            return "Session Completed";
        }
        if self.session_failed.is_some() {
            return "Session Failed";
        }
        "Unknown"
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
                Some(format!(
                    "{}: {}",
                    progress.title,
                    progress.description.as_deref().unwrap_or("")
                ))
            }
        } else {
            self.session_failed
                .as_ref()
                .map(|failed| format!("Session failed: {}", failed.reason))
        }
    }
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
    pub title: String,
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
    #[serde(rename = "unidiffPatch")]
    pub unidiff_patch: String,
    #[serde(rename = "baseCommitId")]
    pub base_commit_id: String,
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
