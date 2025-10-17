#!/usr/bin/env bash
# Helper script to trigger GitHub Actions workflows manually
# Requires: gh (GitHub CLI) - https://cli.github.com/

set -e

REPO="kiwina/gules"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

show_help() {
    cat << EOF
Usage: $(basename "$0") [COMMAND] [OPTIONS]

Commands:
    test            Run the test suite
    build [TARGET]  Build for specific target (or all if not specified)
    release TAG     Create a release with the given tag
    list            List recent workflow runs
    watch [RUN_ID]  Watch a workflow run until completion

Build Targets:
    all             Build for all platforms (default)
    linux-x86_64    Linux x86_64
    linux-aarch64   Linux ARM64
    windows-x86_64  Windows x86_64
    macos-x86_64    macOS Intel
    macos-aarch64   macOS Apple Silicon

Examples:
    $(basename "$0") test
    $(basename "$0") build
    $(basename "$0") build linux-x86_64
    $(basename "$0") release v1.0.0
    $(basename "$0") list
    $(basename "$0") watch 12345678

EOF
}

trigger_test() {
    echo "🧪 Triggering test workflow..."
    gh workflow run test.yml --repo "$REPO"
    echo "✅ Test workflow triggered!"
    echo "📊 View status: gh run list --repo $REPO --limit 5"
}

trigger_build() {
    local target="${1:-all}"
    echo "🔨 Triggering build workflow for target: $target"
    gh workflow run build.yml --repo "$REPO" -f target="$target"
    echo "✅ Build workflow triggered!"
    echo "📊 View status: gh run list --repo $REPO --limit 5"
}

trigger_release() {
    local tag="$1"
    if [ -z "$tag" ]; then
        echo "❌ Error: Release tag required"
        echo "Usage: $(basename "$0") release TAG"
        echo "Example: $(basename "$0") release v1.0.0"
        exit 1
    fi
    
    echo "🚀 Triggering release workflow for tag: $tag"
    gh workflow run release.yml --repo "$REPO" -f tag="$tag" -f create_tag=true
    echo "✅ Release workflow triggered!"
    echo "📊 View status: gh run list --repo $REPO --limit 5"
}

list_runs() {
    echo "📋 Recent workflow runs:"
    gh run list --repo "$REPO" --limit 10
}

watch_run() {
    local run_id="$1"
    if [ -z "$run_id" ]; then
        echo "📊 Getting latest run..."
        run_id=$(gh run list --repo "$REPO" --limit 1 --json databaseId --jq '.[0].databaseId')
    fi
    
    echo "👀 Watching run: $run_id"
    gh run watch "$run_id" --repo "$REPO"
}

# Main command handling
case "${1:-}" in
    test)
        trigger_test
        ;;
    build)
        trigger_build "${2:-all}"
        ;;
    release)
        trigger_release "$2"
        ;;
    list)
        list_runs
        ;;
    watch)
        watch_run "$2"
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "❌ Unknown command: ${1:-}"
        echo ""
        show_help
        exit 1
        ;;
esac
