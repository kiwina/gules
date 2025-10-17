# MCP Server Integration Guide

Run Gules as a Model Context Protocol (MCP) server to integrate Jules AI capabilities with AI assistants like Claude Desktop, VS Code Copilot, and other MCP-compatible tools.

## Overview

Gules provides two MCP server implementations via feature flags:

### Pure SDK Server (`--features mcp`)

- **Tools**: 9 tools (1:1 Jules SDK mapping)
- **Use**: Lightweight, core functionality only
- **Best for**: Basic Jules integration

### Extended Server (`--features extended-mcp`)

- **Tools**: 11 tools (9 SDK + 2 extended)
- **Extended tools**: `watch_session`, `issue_status`
- **Best for**: Advanced monitoring and GitHub integration

## Quick Start

1.  **Install `gules`:**
    ```bash
    # For the full-featured server:
    cargo install --path . --features extended-mcp

    # For the basic SDK-only server:
    cargo install --path . --features mcp
    ```

2.  **Configure API Key:**
    Create `~/.config/gules/config.toml` with your API key:
    ```toml
    api_key = "your-jules-api-key-here"
    ```

3.  **Run the MCP Server:**
    The server is started by your MCP client (e.g., VS Code), which will run the `gules --mcp` command.

## Available Tools

### Pure SDK Tools (9 Tools)

Available in **both** `mcp` and `extended-mcp` builds.

1.  `create_session`: Create a new Jules coding session.
2.  `get_session`: Get details of a specific session.
3.  `list_sessions`: List all sessions with pagination.
4.  `send_message`: Send a message to an active session.
5.  `approve_plan`: Approve a session's execution plan.
6.  `list_sources`: List available code sources.
7.  `get_source`: Get details of a specific source.
8.  `list_activities`: List all activities for a session.
9.  `get_activity`: Get details of a single activity.

### Extended Tools (2 Tools)

Available **only** in the `extended-mcp` build.

1.  `watch_session`: Monitor a session in real-time until it completes or fails.
2.  `issue_status`: Check for Jules sessions linked to a GitHub issue (placeholder, CLI is recommended).

## Client Configuration

### VS Code (with Copilot)

Add the following to your `settings.json`:
```json
"github.copilot.chat.mcp.servers": {
  "gules": {
    "command": "/path/to/your/gules/binary",
    "args": ["--mcp"],
    "env": {}
  }
}
```
*(On Windows with WSL, you may need to use `wsl` as the command and provide the WSL path to the binary.)*

### Claude Desktop

Add the following to `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "gules": {
      "command": "/path/to/your/gules/binary",
      "args": ["--mcp"]
    }
  }
}
```

## Architecture

- **Transport:** The server uses `stdio` for communication, which is the standard for local MCP servers.
- **Lifecycle:** The MCP client is responsible for starting and stopping the `gules --mcp` process.
- **Error Handling:** The server provides detailed JSON-RPC error responses for API failures, validation errors, and internal issues.

## Troubleshooting

- **Server Not Starting:** Ensure the `command` path in your client's configuration points to the correct `gules` binary. Check permissions.
- **API Key Not Found:** Make sure the `api_key` is set in `~/.config/gules/config.toml` or via the `JULES_API_KEY` environment variable.
- **Tools Not Appearing:** Restart your MCP client (e.g., VS Code) after configuring the server. Use an MCP inspector to verify the server's capabilities.

---
**Last Updated:** 2025-10-17
