//! Integration tests for API completeness
//!
//! Tests all required API methods with mocked HTTP responses

use jules_rs::JulesClient;
use mockito::Server;
use serde_json::json;

#[tokio::test]
async fn test_send_message_method() {
    let mut server = Server::new_async().await;

    // Mock the send message endpoint
    let _send_mock = server
        .mock(
            "POST",
            mockito::Matcher::Regex(r"/sessions/.+:sendMessage".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client
        .send_message("session-123", "What should I do next?")
        .await;

    assert!(result.is_ok(), "send_message should succeed");
    _send_mock.assert_async().await;
}

#[tokio::test]
async fn test_approve_plan_method() {
    let mut server = Server::new_async().await;

    // Mock the approve plan endpoint
    let _approve_mock = server
        .mock(
            "POST",
            mockito::Matcher::Regex(r"/sessions/.+:approvePlan".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.approve_plan("session-123").await;

    assert!(result.is_ok(), "approve_plan should succeed");
    _approve_mock.assert_async().await;
}

#[tokio::test]
async fn test_list_sessions_with_pagination() {
    let mut server = Server::new_async().await;

    // Mock list sessions endpoint with pagination
    let _list_mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/sessions\?pageSize=.+".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "sessions": [
                    {
                        "name": "sessions/session-1",
                        "id": "session-1",
                        "prompt": "Build a REST API",
                        "sourceContext": {
                            "source": "sources/repo-1",
                            "githubRepoContext": {
                                "startingBranch": "main"
                            }
                        },
                        "createTime": "2024-01-15T10:00:00Z",
                        "state": "COMPLETED",
                        "url": "https://console.cloud.google.com/sessions/session-1",
                        "outputs": []
                    }
                ],
                "nextPageToken": "token-123"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    // Test with pagination parameters
    let result = client.list_sessions(Some(10), Some("token-456")).await;

    assert!(result.is_ok(), "list_sessions should succeed");

    let response = result.unwrap();
    assert_eq!(response.sessions.len(), 1);
    assert_eq!(response.next_page_token, Some("token-123".to_string()));

    _list_mock.assert_async().await;
}

#[tokio::test]
async fn test_list_sources_with_pagination() {
    let mut server = Server::new_async().await;

    // Mock list sources endpoint with pagination
    let _list_mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/sources\?pageSize=.+".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "sources": [
                    {
                        "name": "sources/repo-1",
                        "id": "repo-1",
                        "githubRepo": {
                            "owner": "myorg",
                            "repo": "my-repo",
                            "isPrivate": false,
                            "defaultBranch": {
                                "displayName": "main"
                            },
                            "branches": []
                        }
                    }
                ],
                "nextPageToken": "token-789"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.list_sources(None, Some(10), Some("token-456")).await;

    assert!(result.is_ok(), "list_sources should succeed");

    let response = result.unwrap();
    assert_eq!(response.sources.len(), 1);
    assert_eq!(response.next_page_token, Some("token-789".to_string()));

    _list_mock.assert_async().await;
}

#[tokio::test]
async fn test_list_activities_with_pagination() {
    let mut server = Server::new_async().await;

    // Mock list activities endpoint with pagination
    let _list_mock = server
        .mock(
            "GET",
            mockito::Matcher::Regex(r"/sessions/.+/activities\?pageSize=.+".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "activities": [
                    {
                        "name": "sessions/session-1/activities/activity-1",
                        "id": "activity-1",
                        "description": "Started planning",
                        "createTime": "2024-01-15T10:05:00Z",
                        "originator": "system",
                        "artifacts": [],
                        "planGenerated": {
                            "plan": {
                                "id": "plan-1",
                                "steps": [],
                                "createTime": "2024-01-15T10:05:00Z"
                            }
                        }
                    }
                ],
                "nextPageToken": "activity-token-123"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client
        .list_activities("session-1", Some(10), Some("token-456"))
        .await;

    assert!(result.is_ok(), "list_activities should succeed");

    let response = result.unwrap();
    assert_eq!(response.activities.len(), 1);
    assert_eq!(
        response.next_page_token,
        Some("activity-token-123".to_string())
    );

    _list_mock.assert_async().await;
}

#[tokio::test]
async fn test_send_message_error_handling() {
    let mut server = Server::new_async().await;

    // Mock error response
    let _error_mock = server
        .mock(
            "POST",
            mockito::Matcher::Regex(r"/sessions/.+:sendMessage".to_string()),
        )
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": {
                    "code": 404,
                    "message": "Session not found",
                    "status": "NOT_FOUND"
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.send_message("nonexistent-session", "test").await;

    assert!(
        result.is_err(),
        "send_message should fail for invalid session"
    );
    _error_mock.assert_async().await;
}

#[tokio::test]
async fn test_approve_plan_error_handling() {
    let mut server = Server::new_async().await;

    // Mock error response
    let _error_mock = server
        .mock(
            "POST",
            mockito::Matcher::Regex(r"/sessions/.+:approvePlan".to_string()),
        )
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": {
                    "code": 400,
                    "message": "No plan to approve",
                    "status": "INVALID_ARGUMENT"
                }
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.approve_plan("session-without-plan").await;

    assert!(
        result.is_err(),
        "approve_plan should fail when no plan exists"
    );
    _error_mock.assert_async().await;
}

#[tokio::test]
async fn test_pagination_with_no_token() {
    let mut server = Server::new_async().await;

    // Mock list sessions without page token
    let _list_mock = server
        .mock("GET", "/sessions?pageSize=20")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "sessions": [],
                "nextPageToken": null
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    let result = client.list_sessions(Some(20), None).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.next_page_token.is_none());

    _list_mock.assert_async().await;
}

#[tokio::test]
async fn test_default_page_size() {
    let mut server = Server::new_async().await;

    // Mock list sessions with default page size (30)
    let _list_mock = server
        .mock("GET", "/sessions?pageSize=30")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "sessions": [],
                "nextPageToken": null
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = JulesClient::with_config(jules_rs::JulesConfig {
        api_key: "test-key".to_string(),
        base_url: server.url(),
    });

    // Call without page size - should use default 30
    let result = client.list_sessions(None, None).await;

    assert!(result.is_ok());
    _list_mock.assert_async().await;
}
