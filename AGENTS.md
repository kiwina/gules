# Gules Project - AI Agent Guide

**Last Updated**: October 27, 2025  
**Status**: ✅ Production Ready

## Quick Reference

### Architecture
```
jules-rs   → Pure SDK (9 API methods)
jules-cli  → CLI interface (13 commands)
jules-mcp  → MCP server (9 tools)
gules      → Extended CLI (20 commands) + optional MCP (11 tools)
```

### Project Structure
```
crates/
├── jules-rs/          SDK (9 API methods)
├── jules-core/        Shared utilities
├── jules-cli/         CLI (13 commands)
├── jules-mcp/         MCP server (9 tools)
└── gules/             Extended CLI + MCP
```

### Test Status
- **Total**: 98 tests passing
- **Coverage**: 100% API coverage
- **Quality**: Zero warnings, zero errors

## Commands (20 total)

**Session Management** (5): sessions, session, active, completed, failed  
**Session Actions** (3): create, send-message, approve-plan  
**Sources** (2): sources, source  
**Activities** (2): activities, activity  
**Activity Filtering** (1): filter-activities  
**Cache Management** (3): cache stats, cache clear, cache delete  
**Extended** (4): watch, monitor, issue-status, pr-status  
**Config** (1): config

## Development

### Workflow

1. Read crate README for context
2. Follow existing patterns (commands in `src/commands/`)
3. Use mockito for HTTP mocking (never real API)
4. Add tests for new features
5. Update documentation
6. Verify: `cargo test --all && cargo clippy`

### Guidelines

- ✅ Use mockito for tests (never real API)
- ✅ Update docs with changes
- ✅ Test error paths
- ❌ Don't hardcode credentials
- ❌ Don't skip error handling

### Quick Commands

```bash
cargo test --all          # Run all tests
cargo clippy --all        # Check code quality
cargo build --release     # Build production binary
```

## Key Principles

**jules-rs** = Pure 1:1 SDK (9 API methods, no convenience helpers)  
**gules** = Extended features (convenience wrappers + GitHub integrations)

Keep SDK independently publishable. Add enhancements to gules, not SDK.

---

**Repository**: github.com/kiwina/gules  
**Status**: Production Ready ✅
