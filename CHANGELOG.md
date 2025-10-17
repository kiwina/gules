# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete CLI with 17 commands covering all Jules SDK operations
- MCP server support via feature flags (`mcp` and `extended-mcp`)
- Extended commands: `watch`, `monitor`, `issue-status`, `pr-status`
- GitHub integration for issue and PR tracking
- Configuration management via `~/.config/gules/config.toml`

### Features
- **jules-rs**: Pure 1:1 SDK with 9 API methods
- **jules-cli**: 13 commands (sessions, sources, activities, config)
- **jules-mcp**: MCP server with 9 SDK tools
- **gules**: Extended CLI + optional MCP (11 tools with `extended-mcp`)

### Build Options
- Default: CLI only (lightweight)
- `--features mcp`: CLI + basic MCP server
- `--features extended-mcp`: CLI + extended MCP with monitoring

---

## [0.1.0] - 2025-10-17

Initial release of Gules - Jules AI CLI and MCP server.

### Components
- jules-rs (SDK)
- jules-core (shared utilities)
- jules-cli (pure SDK CLI)
- jules-mcp (pure SDK MCP server)
- gules (extended CLI with optional MCP)

### Statistics
- 46 tests passing
- Zero warnings
- Production ready
