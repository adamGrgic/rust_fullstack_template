.PHONY: help setup setup-db clean-db migrate api api-logs client client-logs logs build-api build-client test fmt lint clean stop compile-themes export-themes

help: ## Show this help message
	@echo "Atom Platform - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Run initial setup (tools and dependencies)
	@./setup.sh

setup-db: ## Setup PostgreSQL database and user
	@./setup_db.sh

clean-db: ## Remove PostgreSQL database and user (DESTRUCTIVE)
	@./clean_db.sh

migrate: ## Run database migrations
	@cd platform/api && cargo sqlx migrate run

api: ## Run API server
	@cd platform/api && cargo run

api-logs: ## Tail API logs
	@./logs.sh api

client: ## Run client dev server
	@cd client && unset NO_COLOR && trunk serve

client-logs: ## Tail client logs
	@./logs.sh client

logs: ## Tail all logs
	@./logs.sh both

build-api: ## Build API for production
	@cd platform/api && cargo build --release
	@echo "✅ API built: platform/api/target/release/platform-api"

build-client: ## Build client for production
	@cd client && unset NO_COLOR && trunk build --release
	@echo "✅ Client built: client/dist/"

test: ## Run all tests
	@cargo test --all

fmt: ## Format all code
	@cargo fmt --all
	@echo "✅ Code formatted"

lint: ## Run clippy on all code
	@cargo clippy --all -- -D warnings

clean: ## Clean build artifacts
	@cargo clean
	@cd client && unset NO_COLOR && trunk clean
	@echo "✅ Cleaned build artifacts"

stop: ## Stop all running services (ports 8080 and 8081)
	@echo "Stopping services on ports 8080 and 8081..."
	@-lsof -ti:8080 | xargs -r kill 2>/dev/null || true
	@-lsof -ti:8081 | xargs -r kill 2>/dev/null || true
	@sleep 1
	@if lsof -ti:8080,8081 > /dev/null 2>&1; then \
		echo "⚠️  Some processes still running, force killing..."; \
		lsof -ti:8080,8081 | xargs -r kill -9 2>/dev/null || true; \
		sleep 1; \
	fi
	@echo "✅ Services stopped"

compile-themes: ## Compile JSON theme files to database
	@./compile_themes.sh

export-themes: ## Export database themes to JSON files
	@./export_themes.sh

dev: ## Start development environment (API + client)
	@./dev.sh

