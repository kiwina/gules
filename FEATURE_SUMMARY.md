# Activity Filtering & Caching Feature

## Overview
Added advanced activity filtering with local caching system for efficient querying and offline access.

## New Commands (3 total)

### 1. `filter-activities` - Smart Activity Filtering
```bash
# Get the last activity
gules filter-activities <SESSION_ID> --last 1

# Get last 5 activities
gules filter-activities <SESSION_ID> --last 5

# Get only failed activities
gules filter-activities <SESSION_ID> --type failed

# Get last error/failed activity
gules filter-activities <SESSION_ID> --last 1 --type error

# Get activities with bash output (test errors!)
gules filter-activities <SESSION_ID> --has-bash-output

# Get last bash output (perfect for debugging test failures)
gules filter-activities <SESSION_ID> --last 1 --has-bash-output --format full

# Filter multiple types
gules filter-activities <SESSION_ID> --type agent-message,user-message

# Get full details for last 3 activities
gules filter-activities <SESSION_ID> --last 3 --format full

# Export to JSON
gules filter-activities <SESSION_ID> --type error --format json

# Content only (no formatting)
gules filter-activities <SESSION_ID> --last 1 --format content-only
```

**Activity Types:**
- `agent-message` / `agent` - Messages from Jules
- `user-message` / `user` - User messages
- `plan` / `plan-generated` - Generated plans
- `plan-approved` / `approved` - Approved plans
- `progress` / `progress-updated` - Progress updates
- `completed` / `session-completed` - Completion events
- `failed` / `session-failed` / `error` - Failure events

**Output Formats:**
- `table` (default) - Formatted table view
- `json` - JSON output for piping/scripting
- `full` - Full details including artifacts
- `content-only` - Just the content text

### 2. `cache stats` - View Cache Statistics
```bash
gules cache stats
```

Shows:
- Cache status (enabled/disabled)
- Cache location
- Number of cached sessions
- Total activities cached
- Disk usage
- List of cached sessions with last update time

### 3. `cache clear` - Clear All Cache
```bash
gules cache clear
```

### 4. `cache delete` - Delete Specific Session Cache
```bash
gules cache delete <SESSION_ID>
```

## Implementation Details

### Caching System
- **Location**: `~/.cache/gules/activities/`
- **Format**: JSON files per session
- **Max Sessions**: 50 (configurable via `~/.config/gules/config.toml`)
- **Eviction**: FIFO (oldest first)
- **Updates**: Incremental using page tokens

### How It Works

1. **First fetch**: Retrieves all activities (up to 100) and caches them
2. **Subsequent fetches**: Uses page tokens to get only new activities
3. **Merging**: Deduplicates by activity ID, sorts by creation time
4. **Filtering**: Client-side filtering (instant, no API calls)

### Configuration

Add to `~/.config/gules/config.toml`:

```toml
[cache]
enabled = true
max_sessions = 50
```

## Use Cases

### 1. Quick Error Debugging
```bash
# Get the last error with full details
gules filter-activities session-123 --last 1 --type error --format full

# Get last test failure output
gules filter-activities session-123 --last 1 --has-bash-output --format full
```

### 2. Fast Iteration Loop
```bash
# 1. See what failed
gules filter-activities session-123 --type error --last 1

# 2. Fix and send message to Jules
gules send-message session-123 "Fixed the test issue by..."

# 3. Monitor progress
gules watch session-123
```

### 3. Review Conversation
```bash
# Get all messages
gules filter-activities session-123 --type agent-message,user-message

# Get just the content
gules filter-activities session-123 --type agent-message --format content-only
```

### 4. Export for Analysis
```bash
# Export all activities to JSON
gules filter-activities session-123 --format json > activities.json

# Export only errors
gules filter-activities session-123 --type error --format json > errors.json
```

## Benefits

1. **⚡ Fast**: Client-side filtering, no repeated API calls
2. **💾 Efficient**: Incremental updates, only fetches new data
3. **🔌 Offline**: Works with cached data when API unavailable
4. **🎯 Targeted**: Filter by type, bash output, or last N items
5. **📊 Flexible**: Multiple output formats for different use cases
6. **🧹 Managed**: Auto-eviction with FIFO, manual cache control

## Testing

All existing tests pass (46/46):
- 32 extended command tests
- 9 API completeness tests  
- 4 integration tests
- 1 doc test

## Command Count

- **Before**: 17 commands
- **After**: 20 commands (+3)

## Architecture

New modules:
- `jules-core/src/activity_cache.rs` - Cache management logic
- `gules/src/commands/filter_activities.rs` - Filtering command
- `gules/src/commands/cache.rs` - Cache management commands

Updated:
- `jules-core/src/config.rs` - Added cache configuration
- `jules-core/src/lib.rs` - Exported cache module
- `gules/src/main.rs` - Added new commands and CLI args
