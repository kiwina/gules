# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
