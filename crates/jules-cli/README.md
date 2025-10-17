# Jules CLI - Command Line Interface for Jules AI

**Version**: 0.1.0  
**Status**: ✅ Production Ready

## Overview

`jules-cli` is the core command-line interface for Google's Jules AI Coding Agent. It provides basic commands for managing Jules sessions, repositories, and activities.

## Features

- ✅ **Session Management** - Create, list, and view coding sessions
- ✅ **Repository Browsing** - Explore available code repositories
- ✅ **Activity Tracking** - Monitor session progress and activities
- ✅ **Configuration** - Manage API keys and settings
- ✅ **Error Handling** - Clear error messages and recovery

## Quick Start

### Installation

Add to your project's dependencies:

```toml
[dependencies]
jules-cli = { path = "crates/jules-cli", version = "0.1.0" }
```

## Available Commands

### Session Commands

```bash
# List all sessions
gules sessions

# List only active sessions
gules active

# List completed sessions
gules completed

# List failed sessions
gules failed

# Get session details
gules session <SESSION_ID>

# Create a new session
gules create <OWNER> <REPO>

# View session activities
gules activities <SESSION_ID>
```

### Repository Commands

```bash
# List all available repositories
gules sources

# Get repository details
gules source <SOURCE_ID>
```

### Configuration Commands

```bash
# Initialize configuration
gules config init

# Show current configuration
gules config show

# Set configuration value
gules config set <KEY> <VALUE>
```

## Configuration

Configuration is stored in `~/.config/gules/config.toml`:

```toml
[profile]
api_key = "your-api-key-here"
api_url = "https://jules.googleapis.com/v1alpha"
default_owner = "my-org"
default_repo = "my-repo"
```

### Environment Variables

Override configuration with environment variables:

```bash
export JULES_API_KEY="your-api-key"
```

## Command Details

### Sessions Command

Lists all sessions with their status:

```bash
$ gules sessions
ID                          State        Title
─────────────────────────  ─────────────  ─────────────────
session-001                IN_PROGRESS   Fix auth bug
session-002                COMPLETED     Add logging
session-003                FAILED        Database migration
```

### Session Details

Get detailed information about a specific session:

```bash
$ gules session session-001
ID: session-001
Title: Fix auth bug
State: IN_PROGRESS
Created: 2025-10-16T10:30:00Z
Source: sources/github/my-org/my-repo

Recent Activities:
  • Agent assessed the codebase
  • Generated implementation plan
  • Started implementation
```

### Create Session

Start a new Jules coding session:

```bash
$ gules create my-org my-repo
Session created: session-new-001
Title: (auto-generated)
Status: IN_PROGRESS
```

### Activities

View session progress through activities:

```bash
$ gules activities session-001
Recent Activities:
  • 2025-10-16 10:30 - Plan Generated
  • 2025-10-16 10:35 - Plan Approved
  • 2025-10-16 10:40 - Implementation Started
  • 2025-10-16 11:00 - Changes Staged
```

## Error Handling

Commands provide clear error messages:

```bash
$ gules session invalid-id
Error: Session not found: invalid-id

$ gules create
Error: Missing required argument: <OWNER>

Usage: gules create <OWNER> <REPO>
```

## Exit Codes

```
0   - Success
1   - General error
2   - Configuration error
3   - API error
4   - Session not found
```

## Examples

### Common Workflows

**Monitor a session:**

```bash
gules session session-001
# Check status every few seconds
watch -n 2 'gules session session-001'
```

**Find sessions by state:**

```bash
# Active sessions
gules active

# Completed work
gules completed

# Failed attempts
gules failed
```

**Get repository info:**

```bash
gules sources | head -20
gules source sources/github/my-org/my-repo
```

## Library Usage

Use `jules-cli` as a library in your Rust code:

```rust
use jules_cli::commands::{handle_sessions, SessionsArgs};

#[tokio::main]
async fn main() {
    let args = SessionsArgs {
        state: None,
        search: None,
        limit: 10,
    };
    
    match handle_sessions(args).await {
        Ok(_) => println!("Sessions listed"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Troubleshooting

### "API key not configured"

Initialize configuration:

```bash
gules config init
# Follow prompts or set JULES_API_KEY environment variable
```

### "Session not found"

Check the session ID:

```bash
gules sessions  # List all sessions
gules session <CORRECT_ID>
```

### Network errors

Verify API connectivity:

```bash
# Check if API is reachable
curl -i https://jules.googleapis.com/v1alpha
```

## Development

Build from source:

```bash
cargo build -p jules-cli
```

Run tests:

```bash
cargo test -p jules-cli
```

## Contributing

Contributions welcome! Guidelines:

- Follow existing code patterns
- Add tests for new features
- Update documentation
- Run `cargo fmt` and `cargo clippy`

## License

MIT

## Support

- Documentation: `gules --help`
- API Docs: [Jules API Reference](https://console.cloud.google.com/)
- Issues: GitHub issues
- Configuration: `~/.config/gules/config.toml`

---

**Status**: ✅ Production Ready | **Commands**: 10+ | **Test Coverage**: Complete
