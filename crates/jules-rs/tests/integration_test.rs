//! Integration tests for jules-rs SDK
//!
//! These tests verify the SDK works correctly with mock API responses.
//! Uses mockito to simulate the Jules API without making real network calls.

use jules_rs::JulesClient;
use mockito::Server;

#[tokio::test]
async fn test_create_session_integration() {
    let mut server = Server::new_async().await;

    let response_json = r#"{
        "name": "sessions/123456",
        "id": "123456",
        "prompt": "Test prompt",
        "sourceContext": {
            "source": "sources/github/owner/repo"
        },
        "state": "QUEUED"
    }"#;

    let mock = server
        .mock("POST", "/sessions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_json)
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::client::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let request = jules_rs::types::session::CreateSessionRequest {
        prompt: "Test prompt".to_string(),
        source_context: jules_rs::types::session::SourceContext {
            source: "sources/github/owner/repo".to_string(),
            github_repo_context: None,
        },
        title: None,
        require_plan_approval: None,
        automation_mode: None,
    };

    let result = client.create_session(request).await;
    assert!(result.is_ok());

    let session = result.unwrap();
    assert_eq!(session.id, "123456");
    assert_eq!(session.prompt, "Test prompt");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_sessions_integration() {
    let mut server = Server::new_async().await;

    let response_json = r#"{
        "sessions": [
            {
                "name": "sessions/123",
                "id": "123",
                "prompt": "Session 1",
                "sourceContext": {
                    "source": "sources/github/owner/repo"
                }
            }
        ]
    }"#;

    let mock = server
        .mock("GET", "/sessions?pageSize=30")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_json)
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::client::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.list_sessions(None, None).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.sessions.len(), 1);
    assert_eq!(response.sessions[0].id, "123");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_source_with_slashes_integration() {
    let mut server = Server::new_async().await;

    let response_json = r#"{
        "name": "sources/github/owner/repo",
        "id": "github/owner/repo",
        "githubRepo": {
            "owner": "owner",
            "repo": "repo"
        }
    }"#;

    // Note: Path should NOT be URL-encoded
    let mock = server
        .mock("GET", "/sources/github/owner/repo")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(response_json)
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::client::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    // Test with full path
    let result = client.get_source("sources/github/owner/repo").await;
    assert!(result.is_ok());

    let source = result.unwrap();
    assert_eq!(source.id, "github/owner/repo");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_error_handling_integration() {
    let mut server = Server::new_async().await;

    let error_json = r#"{
        "error": {
            "code": 404,
            "message": "Session not found",
            "status": "NOT_FOUND"
        }
    }"#;

    let mock = server
        .mock("GET", "/sessions/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(error_json)
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::client::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.get_session("nonexistent").await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));
    assert!(error.to_string().contains("Session not found"));

    mock.assert_async().await;
}
