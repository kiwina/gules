# Gules Documentation

Complete documentation for the Gules CLI and MCP server for Google's Jules AI coding agent.

## Quick Links

- **[Commands Reference](COMMANDS.md)** - Complete CLI command reference (17 commands)
- **[MCP Integration Guide](MCP.md)** - Model Context Protocol server setup

## Installation

```bash
# CLI only (default)
cargo install --path crates/gules

# With MCP support
cargo install --path crates/gules --features mcp

# With extended MCP (watch_session + issue_status tools)
cargo install --path crates/gules --features extended-mcp
```

## Quick Start

### 1. Configure API Key

Create `~/.config/gules/config.toml`:

```toml
api_key = "your-jules-api-key"
api_url = "https://jules.googleapis.com/v1alpha"
```

Get your API key from: https://jules.googleapis.com

### 2. Create a Session

```bash
gules create "Fix authentication bug" --source sources/github/owner/repo
```

### 3. Monitor Progress

```bash
# Watch in real-time
gules watch <session-id>

# Check status anytime
gules session <session-id>

# List all sessions
gules sessions
```

## Feature Comparison

| Build | Features | Best For |
|-------|----------|----------|
| **Default** | 17 CLI commands | CLI users |
| **--features mcp** | CLI + 9 MCP tools | Claude Desktop, VS Code |
| **--features extended-mcp** | CLI + 11 MCP tools | Advanced MCP integration |

**MCP Tools**: Pure SDK (9) = create_session, get_session, list_sessions, send_message, approve_plan, list_sources, get_source, list_activities, get_activity

**Extended MCP** adds: watch_session, issue_status

## Support

- **Issues**: https://github.com/kiwina/gules/issues
- **Jules API**: https://jules.googleapis.com

---

**Version**: 0.1.0  
**Last Updated**: October 17, 2025
