#!/usr/bin/env bash
# Install git hooks for the gules project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
GIT_HOOKS_DIR="$PROJECT_DIR/.git/hooks"

echo "📦 Installing git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install pre-commit hook
echo "  → Installing pre-commit hook (auto-format)..."
ln -sf "$SCRIPT_DIR/pre-commit.sh" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"

# Install pre-push hook
echo "  → Installing pre-push hook (format check + lint + test)..."
ln -sf "$SCRIPT_DIR/pre-push.sh" "$GIT_HOOKS_DIR/pre-push"
chmod +x "$GIT_HOOKS_DIR/pre-push"

echo ""
echo "✅ Git hooks installed successfully!"
echo ""
echo "Hooks installed:"
echo "  • pre-commit: Auto-formats code before commits"
echo "  • pre-push: Runs format check, clippy, and tests before pushing"
echo ""
echo "To skip hooks temporarily:"
echo "  git commit --no-verify"
echo "  git push --no-verify"
