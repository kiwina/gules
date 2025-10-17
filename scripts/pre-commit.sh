#!/usr/bin/env bash
# Pre-commit hook: Auto-format code
# This runs quickly before each commit to ensure consistent formatting

set -e

echo "🎨 Auto-formatting code..."

# Format all Rust code
cargo fmt --all

# Stage formatted files
git add -u

echo "✅ Code formatted successfully!"
