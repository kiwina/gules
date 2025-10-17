.PHONY: help build test run clean install fmt lint check dev setup-hooks ci-test ci-build ci-release ci-list ci-watch

# Default target
help:
	@echo "Gules - Jules AI CLI Tool"
	@echo ""
	@echo "Setup:"
	@echo "  make setup-hooks - Install git hooks (pre-commit + pre-push)"
	@echo ""
	@echo "Development Commands:"
	@echo "  make build       - Build the project in release mode"
	@echo "  make test        - Run all tests"
	@echo "  make run         - Run the CLI (with MCP feature)"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make install     - Install gules binary to ~/.cargo/bin"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt         - Format code with rustfmt"
	@echo "  make lint        - Run clippy lints"
	@echo "  make check       - Quick compile check (no build)"
	@echo "  make dev         - Run all checks (fmt + lint + test)"
	@echo ""
	@echo "GitHub Actions (requires gh CLI):"
	@echo "  make ci-test     - Trigger test workflow on GitHub"
	@echo "  make ci-build    - Trigger build workflow for all platforms"
	@echo "  make ci-release  - Trigger release workflow (usage: make ci-release TAG=v1.0.0)"
	@echo "  make ci-list     - List recent workflow runs"
	@echo "  make ci-watch    - Watch latest workflow run"
	@echo ""

# Setup
setup-hooks:
	@./scripts/install-hooks.sh

# Development
build:
	cargo build --release --features mcp

test:
	cargo test --all --features mcp -- --nocapture

run:
	cargo run --features mcp -- $(ARGS)

clean:
	cargo clean

install: build
	cargo install --path . --features mcp

# Code Quality
fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all --features mcp -- -D warnings

check:
	cargo check --all --features mcp

dev: fmt-check lint test
	@echo "✅ All checks passed!"

# GitHub Actions
ci-test:
	@./scripts/trigger-workflow.sh test

ci-build:
	@./scripts/trigger-workflow.sh build

ci-build-linux-x86:
	@./scripts/trigger-workflow.sh build linux-x86_64

ci-build-linux-arm:
	@./scripts/trigger-workflow.sh build linux-aarch64

ci-build-windows:
	@./scripts/trigger-workflow.sh build windows-x86_64

ci-build-macos-intel:
	@./scripts/trigger-workflow.sh build macos-x86_64

ci-build-macos-arm:
	@./scripts/trigger-workflow.sh build macos-aarch64

ci-release:
ifndef TAG
	@echo "❌ Error: TAG is required"
	@echo "Usage: make ci-release TAG=v1.0.0"
	@exit 1
endif
	@./scripts/trigger-workflow.sh release $(TAG)

ci-list:
	@./scripts/trigger-workflow.sh list

ci-watch:
	@./scripts/trigger-workflow.sh watch
