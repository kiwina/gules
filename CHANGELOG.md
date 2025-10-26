# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

---

## [0.2.4] - 2025-10-27

### Fixed
- **SDK Resilience**: Made 11 non-critical String fields optional across jules-rs types
  - **Activity fields**: `AgentMessaged.agent_message`, `UserMessaged.user_message`, `SessionFailed.reason`
  - **BashOutput fields**: `command`, `output`
  - **Media fields**: `data`, `mime_type`
  - **PullRequest fields**: `url`, `title`, `description`
  - **PlanStep fields**: `title`
  - Prevents deserialization failures when Google's Jules API omits documented "required" fields
  - All fields now handled gracefully with descriptive fallback markers

### Changed
- Updated display code across all crates with graceful fallback pattern: `field.as_deref().unwrap_or("[Default]")`
- Descriptive markers for missing data: `[Empty message]`, `[No title]`, `[No output]`, `[Unknown reason]`
- Makes missing data obvious to users while preventing crashes

### Internal
- jules-rs: 0.1.2 → 0.1.3
- jules-core: 0.1.2 → 0.1.3
- gules: 0.2.3 → 0.2.4
- All 73 tests updated and passing

---

## [0.2.2] - 2025-10-26

### Fixed
- **Critical API Compatibility**: Made `GitPatch` fields (`unidiffPatch`, `baseCommitId`) optional
  - Fixes deserialization failures when API returns `GitPatch` objects without these fields
  - Prevents crashes when API response structure varies
  - All commands now resilient to missing fields in API responses
- Updated display code to gracefully handle missing git patch data
- Fixed all tests to work with real API data format

### Changed
- Test data format now uses direct activities array (not wrapped in response object)
- Tests validate against real API responses for better compatibility

---

## [0.2.1] - 2025-10-23

### Added
- **Universal Output Format Support**: All commands now support `--format` flag
  - `json` (default) - Native JSON output for machine-readable results
  - `table` - Human-readable table format
  - `full` - Complete detailed view with all fields
- Default to JSON output for scriptability and piping
- `filter-activities` includes `content-only` format for extracting just text

### Changed
- **Create command defaults**: More sensible defaults for common workflows
  - `--require-approval` now defaults to `false` (was required flag)
  - `--automation-mode` now defaults to `AUTO_CREATE_PR` (was optional)
- All list/get commands output JSON by default for machine-readability
- Improved activity table display with comfy-table (3-column layout)

---

## [0.2.0] - 2025-10-22

### Added
- **Activity Filtering & Caching System**: Smart client-side filtering with local cache
  - `filter-activities` command - Filter activities by type, bash output, or last N items
  - `cache` command - Manage activity cache (stats, clear, delete)
  - Incremental updates using page tokens for efficiency
  - FIFO cache eviction (max 50 sessions, configurable)
  - Offline access to cached activities

### Fixed
- **BashOutput.exit_code**: Made optional to handle missing API fields
  - Prevents deserialization failures in production
  - Displays "unknown" when exit code is missing
  - Added comprehensive validation tests with real API data

### Changed
- Added `--version` flag support to CLI
- Improved error handling in cache operations
- Enhanced display formatting for optional fields

### Testing
- Added 6 new validation tests (71 total, was 65)
- Added real API data validation suite
- Added cache functionality tests
- Zero warnings, zero errors

### Performance
- Batched cache eviction to reduce I/O operations
- Fixed disk size units (proper MiB calculation)
- Optimized filtering with in-place truncation

### Documentation
- Updated README with cache configuration
- Added validation test documentation in jules-rs/README.md
- Documented all activity and artifact types

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
