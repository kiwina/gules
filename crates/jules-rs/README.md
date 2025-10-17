# Jules API SDK

> Pure Rust SDK for Google's Jules AI coding agent API

[![Crates.io](https://img.shields.io/crates/v/jules-api-sdk.svg)](https://crates.io/crates/jules-api-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Documentation](https://docs.rs/jules-api-sdk/badge.svg)](https://docs.rs/jules-api-sdk)

A minimal, type-safe Rust SDK for the Jules AI API. Perfect for integrating Jules into your Rust applications without CLI bloat.

## ✨ Features

- **Pure API Client**: No CLI, no external dependencies
- **Type Safe**: Full Rust type system with serde
- **Async First**: Built on tokio for async operations
- **Minimal Dependencies**: Only essential crates
- **Well Documented**: Comprehensive API docs and examples

## 🚀 Quick Start

### Installation

```bash
cargo add jules-api-sdk
```

### Basic Usage

```rust
use jules_api_sdk::JulesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with API key
    let client = JulesClient::new("your-api-key-here")?;

    // List all sessions
    let sessions = client.list_sessions().await?;
    println!("Found {} sessions", sessions.len());

    // Create a new session
    let session = client
        .create_session("Fix the bug in auth.rs", "sources/github/myorg/myrepo")
        .await?;
    println!("Created session: {}", session.id);

    // Get session details
    let details = client.get_session(&session.id).await?;
    println!("Status: {:?}", details.state);

    Ok(())
}
```

## 📚 API Overview

### Sessions API

```rust
// List all sessions
let sessions = client.list_sessions().await?;

// List with pagination
let sessions = client.list_sessions_with_options(ListSessionsOptions {
    page_size: Some(50),
    state: Some(SessionState::Active),
}).await?;

// Get specific session
let session = client.get_session("session-id").await?;

// Create new session
let session = client.create_session(
    "Fix authentication bug",
    "sources/github/myorg/myrepo"
).await?;

// Get session activities
let activities = client.list_activities("session-id").await?;
```

### Sources API

```rust
// List all sources
let sources = client.list_sources().await?;

// Get specific source
let source = client.get_source("source-id").await?;
```

## 🔧 Configuration

### Environment Variables

```bash
export JULES_API_KEY="your-api-key-here"
export JULES_API_URL="https://jules.googleapis.com/v1alpha"  # Optional
```

### Programmatic Configuration

```rust
use jules_api_sdk::{JulesClient, JulesConfig};

let config = JulesConfig {
    api_key: "your-api-key".to_string(),
    base_url: "https://jules.googleapis.com/v1alpha".to_string(),
};

let client = JulesClient::with_config(config)?;
```

## 📖 Examples

See the [`examples/`](examples/) directory for complete examples:

- [`list_sessions.rs`](examples/list_sessions.rs) - List and filter sessions
- [`create_session.rs`](examples/create_session.rs) - Create and monitor a session
- [`session_activities.rs`](examples/session_activities.rs) - View session activity logs

Run examples:

```bash
# Set your API key
export JULES_API_KEY="your-key-here"

# Run an example
cargo run --example list_sessions
```

## 🏗️ Architecture

```
jules-api-sdk/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── client.rs           # HTTP client implementation
│   ├── types/              # API data types
│   │   ├── mod.rs
│   │   ├── session.rs      # Session types
│   │   ├── source.rs       # Source types
│   │   ├── activity.rs     # Activity types
│   │   └── common.rs       # Shared types
│   └── api/                # API endpoint implementations
│       ├── mod.rs
│       ├── sessions.rs     # Sessions API
│       ├── sources.rs      # Sources API
│       └── activities.rs   # Activities API
├── examples/               # Usage examples
└── tests/                  # Unit and integration tests
```

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run with integration tests (requires API key)
cargo test --features integration-tests
```

## 📋 Error Handling

The SDK uses `anyhow::Result` for error handling:

```rust
use jules_api_sdk::JulesClient;

let client = JulesClient::new("invalid-key")?;
match client.list_sessions().await {
    Ok(sessions) => println!("Sessions: {:?}", sessions),
    Err(e) => eprintln!("Error: {}", e),
}
```

## 🤝 Contributing

This SDK is part of the modular Gules ecosystem. See the workspace [REFACTORING_PLAN.md](../REFACTORING_PLAN.md) for development guidelines.

## 📜 License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🔗 Links

- **Jules AI**: [jules.google.com](https://jules.google.com)
- **API Documentation**: [jules.google.com/docs](https://jules.google.com/docs)
- **Gules Workspace**: [../README.md](../README.md)

---

**Version**: 0.1.0  
**Status**: Active Development  
**Last Updated**: October 16, 2025