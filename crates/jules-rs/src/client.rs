use anyhow::{Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::types::error::ApiError;

/// Configuration for JulesClient
#[derive(Clone, Debug)]
pub struct JulesConfig {
    pub api_key: String,
    pub base_url: String,
}

impl Default for JulesConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://jules.googleapis.com/v1alpha".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct JulesClient {
    client: Client,
    config: JulesConfig,
}

impl JulesClient {
    /// Create a new client with API key (uses default base URL)
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(JulesConfig {
            api_key: api_key.into(),
            ..Default::default()
        })
    }

    /// Create a new client with full configuration
    pub fn with_config(config: JulesConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &JulesConfig {
        &self.config
    }

    /// List sessions with pagination
    /// Maps directly to GET /sessions endpoint
    pub async fn list_sessions(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<crate::types::session::ListSessionsResponse> {
        let mut endpoint = format!("/sessions?pageSize={}", page_size.unwrap_or(30));
        if let Some(token) = page_token {
            endpoint.push_str(&format!("&pageToken={}", token));
        }
        self.get(&endpoint).await
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<crate::types::session::Session> {
        self.get(&format!("/sessions/{}", session_id)).await
    }

    /// Send a message to a session
    pub async fn send_message(&self, session_id: &str, prompt: &str) -> Result<()> {
        use crate::types::session::SendMessageRequest;
        let request = SendMessageRequest {
            prompt: prompt.to_string(),
        };
        let _: serde_json::Value = self
            .post(&format!("/sessions/{}:sendMessage", session_id), &request)
            .await?;
        Ok(())
    }

    /// Approve a plan in a session
    pub async fn approve_plan(&self, session_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .post_empty(&format!("/sessions/{}:approvePlan", session_id))
            .await?;
        Ok(())
    }

    /// Create a new session
    /// Maps directly to POST /sessions endpoint
    pub async fn create_session(
        &self,
        request: crate::types::session::CreateSessionRequest,
    ) -> Result<crate::types::session::Session> {
        self.post("/sessions", &request).await
    }

    /// List sources with optional filter and pagination
    /// Maps directly to GET /sources endpoint
    pub async fn list_sources(
        &self,
        filter: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<crate::types::source::ListSourcesResponse> {
        let mut endpoint = format!("/sources?pageSize={}", page_size.unwrap_or(30));
        if let Some(f) = filter {
            endpoint.push_str(&format!("&filter={}", urlencoding::encode(f)));
        }
        if let Some(token) = page_token {
            endpoint.push_str(&format!("&pageToken={}", token));
        }
        self.get(&endpoint).await
    }

    /// Get a source by ID
    /// Note: source_id should include the full path (e.g., "sources/github/owner/repo")
    /// The API expects forward slashes to NOT be URL-encoded per gRPC Transcoding syntax
    pub async fn get_source(&self, source_id: &str) -> Result<crate::types::source::Source> {
        // Remove 'sources/' prefix if present to avoid duplication
        let source_path = source_id.strip_prefix("sources/").unwrap_or(source_id);
        self.get(&format!("/sources/{}", source_path)).await
    }

    /// List activities for a session with pagination
    /// Maps directly to GET /sessions/{parent}/activities endpoint
    pub async fn list_activities(
        &self,
        session_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<crate::types::activity::ListActivitiesResponse> {
        let mut endpoint = format!(
            "/sessions/{}/activities?pageSize={}",
            session_id,
            page_size.unwrap_or(30)
        );
        if let Some(token) = page_token {
            endpoint.push_str(&format!("&pageToken={}", token));
        }
        self.get(&endpoint).await
    }

    /// Get a single activity by ID
    pub async fn get_activity(
        &self,
        session_id: &str,
        activity_id: &str,
    ) -> Result<crate::types::activity::Activity> {
        self.get(&format!(
            "/sessions/{}/activities/{}",
            session_id, activity_id
        ))
        .await
    }

    /// Generic GET request
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("X-Goog-Api-Key", &self.config.api_key)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// Generic POST request
    pub async fn post<Req: Serialize, Res: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &Req,
    ) -> Result<Res> {
        let url = format!("{}{}", self.config.base_url, endpoint);

        let response = self
            .client
            .post(&url)
            .header("X-Goog-Api-Key", &self.config.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// POST with empty body
    pub async fn post_empty<Res: DeserializeOwned>(&self, endpoint: &str) -> Result<Res> {
        let url = format!("{}{}", self.config.base_url, endpoint);

        let response = self
            .client
            .post(&url)
            .header("X-Goog-Api-Key", &self.config.api_key)
            .header("Content-Length", "0")
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// Handle response with error parsing
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            // Get the response text first
            let body_text = response.text().await.unwrap_or_default();

            // Try to parse as structured error
            if let Ok(api_error) = serde_json::from_str::<ApiError>(&body_text) {
                anyhow::bail!(
                    "API error {}: {} ({})",
                    api_error.error.code,
                    api_error.error.message,
                    api_error.error.status
                );
            } else if !body_text.is_empty() {
                anyhow::bail!("API error {}: {}", status, body_text);
            } else {
                anyhow::bail!("API error: HTTP {}", status);
            }
        }

        response
            .json()
            .await
            .context("Failed to parse response as JSON")
    }
}
