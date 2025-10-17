#!/usr/bin/env bash
# Pre-push hook: Run format check, lint, and tests
# This runs before pushing to ensure code quality

set -e

echo "🚀 Running pre-push checks..."
echo ""

# 1. Format check
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting issues detected!"
    echo "💡 Run 'cargo fmt --all' to fix formatting"
    exit 1
fi
echo "✅ Format check passed!"
echo ""

# 2. Clippy (lint)
echo "🔍 Running clippy lints..."
if ! cargo clippy --all --features mcp -- -D warnings; then
    echo "❌ Clippy lints failed!"
    echo "💡 Fix the warnings above before pushing"
    exit 1
fi
echo "✅ Clippy passed!"
echo ""

# 3. Tests
echo "🧪 Running tests..."
if ! cargo test --all --features mcp; then
    echo "❌ Tests failed!"
    echo "💡 Fix the failing tests before pushing"
    exit 1
fi
echo "✅ Tests passed!"
echo ""

echo "🎉 All pre-push checks passed! Safe to push."
