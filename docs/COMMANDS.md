# Gules CLI Commands Reference

Complete reference for all `gules` CLI commands. This document reflects the full implementation, providing 100% coverage of the Jules SDK plus extended functionality.

## Table of Contents

- [Session Management](#session-management)
  - [create](#create) - Create a new Jules session
  - [sessions](#sessions) - List all sessions
  - [session](#session) - Get session details
  - [send-message](#send-message) - Send a message to a session
  - [approve-plan](#approve-plan) - Approve a session plan
- [Session Filters](#session-filters)
  - [active](#active) - Show active sessions
  - [completed](#completed) - Show completed sessions
  - [failed](#failed) - Show failed sessions
- [Source Management](#source-management)
  - [sources](#sources) - List available code sources
  - [source](#source) - Get source details
- [Activity Management](#activity-management)
  - [activities](#activities) - List session activities
  - [activity](#activity) - Get single activity details
- [Extended Commands](#extended-commands)
  - [watch](#watch) - Watch a session in real-time
  - [monitor](#monitor) - Monitor all active sessions
  - [issue-status](#issue-status) - Link GitHub issues to Jules sessions
  - [pr-status](#pr-status) - Find the session that created a PR
- [Configuration](#configuration)
  - [config](#config) - Manage CLI configuration
- [MCP Server](#mcp-server)
  - [mcp](#mcp-flag) - Run as an MCP server

---

## Session Management

### `create`

Create a new Jules AI coding session.

**Usage:**
```bash
gules create <PROMPT> --source <SOURCE> [OPTIONS]
```

**Arguments:**
- `PROMPT` - The task description for Jules.

**Options:**
- `--source <SOURCE>` - **(required)** Code source (e.g., `sources/github/owner/repo`).
- `--title <TITLE>` - Custom session title.
- `--branch <BRANCH>` - Starting branch (default: `main`).
- `--require-approval` - Require plan approval before execution.
- `--automation-mode <MODE>` - `AUTO_CREATE_PR` or `MANUAL` (default).

**SDK Method:** `create_session(CreateSessionRequest)`

---

### `sessions`

List all Jules sessions with filtering and pagination.

**Usage:**
```bash
gules sessions [OPTIONS]
```

**Options:**
- `--state <STATE>` - Filter by state (`ACTIVE`, `COMPLETED`, `FAILED`).
- `--search <TERM>` - Search in titles and prompts.
- `--limit <NUM>` - Maximum number of results (default: 50).

**SDK Method:** `list_sessions(page_size, page_token)`

---

### `session`

Get detailed information about a specific session.

**Usage:**
```bash
gules session <SESSION_ID>
```

**SDK Method:** `get_session(session_id)`

---

### `send-message`

Send a message to an active Jules session.

**Usage:**
```bash
gules send-message <SESSION_ID> <MESSAGE>
```

**SDK Method:** `send_message(session_id, message)`

---

### `approve-plan`

Approve the execution plan for a session that is `AWAITING_PLAN_APPROVAL`.

**Usage:**
```bash
gules approve-plan <SESSION_ID>
```

**SDK Method:** `approve_plan(session_id)`

---

## Session Filters

These are convenience commands that are client-side filters on top of `sessions`.

### `active`

Show all currently in-progress sessions.

**Usage:**
```bash
gules active [OPTIONS]
```

---

### `completed`

Show recently completed sessions.

**Usage:**
```bash
gules completed [OPTIONS]
```

---

### `failed`

Show failed sessions with error details.

**Usage:**
```bash
gules failed [OPTIONS]
```

---

## Source Management

### `sources`

List available code sources (e.g., GitHub repos).

**Usage:**
```bash
gules sources [OPTIONS]
```

**Options:**
- `--filter <FILTER>` - AIP-160 filter expression.
- `--limit <NUM>` - Maximum number of results (default: 50).

**SDK Method:** `list_sources(filter, page_size, page_token)`

---

### `source`

Get detailed information about a specific source.

**Usage:**
```bash
gules source <SOURCE_ID>
```

**SDK Method:** `get_source(source_id)`

---

## Activity Management

### `activities`

List all activities for a specific session.

**Usage:**
```bash
gules activities <SESSION_ID> [OPTIONS]
```

**SDK Method:** `list_activities(session_id, page_size, page_token)`

---

### `activity`

Get detailed information about a single activity within a session.

**Usage:**
```bash
gules activity <SESSION_ID> <ACTIVITY_ID>
```

**SDK Method:** `get_activity(session_id, activity_id)`

---

## Extended Commands

These commands provide functionality beyond the core Jules SDK.

### `watch`

Continuously monitor a session in real-time until it completes or fails.

**Usage:**
```bash
gules watch <SESSION_ID> --interval <SECONDS>
```

---

### `monitor`

Continuously monitor all active sessions in a live dashboard view.

**Usage:**
```bash
gules monitor --interval <SECONDS>
```

---

### `issue-status`

Check which Jules sessions are linked to a GitHub issue. **Requires `gh` CLI.**

**Usage:**
```bash
gules issue-status <ISSUE_NUM> --owner <OWNER> --repo <REPO>
```

---

### `pr-status`

Find the GitHub PR created by a Jules session.

**Usage:**
```bash
gules pr-status <SESSION_ID>
```

---

## Configuration

### `config`

Manage `gules` CLI configuration.

**Usage:**
```bash
gules config <ACTION>
```

**Actions:**
- `init` - Create a default config file.
- `show` - Display the current configuration.
- `set <KEY> <VALUE>` - Set a configuration value (`api_key`, `api_url`, `default_owner`, `default_repo`).

---

## MCP Server

### `mcp` (flag)

Run `gules` as a Model Context Protocol (MCP) server instead of a CLI.

**Usage:**
```bash
gules --mcp
```

**Note:** This flag is only available when compiled with the `mcp` or `extended-mcp` feature flags. See `docs/MCP.md` for details.
