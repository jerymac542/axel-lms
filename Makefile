# Axel LMS - Makefile
# Enterprise-grade Learning Management System

.PHONY: help dev build test lint fmt fmt-check clean migrate seed docker-up docker-down docker-build docker-push install airgap package release

# Default target
.DEFAULT_GOAL := help

# Variables
PROJECT_NAME := axel-lms
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
BUILD_DATE := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")
GIT_COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")
RUST_VERSION := 1.79
PYTHON_VERSION := 3.12
PHP_VERSION := 8.3
NODE_VERSION := 20

# Directories
RUST_DIR := rust
PYTHON_DIR := python
PHP_DIR := php
INFRA_DIR := infra
CONTRACTS_DIR := contracts
DIST_DIR := dist

# Docker
DOCKER_REGISTRY := ghcr.io/apexos
DOCKER_TAG := $(VERSION)

# Colors
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(GREEN)Axel LMS - Build System$(NC)"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*##/ {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# =============================================================================
# Development
# =============================================================================

dev: ## Start all services in development mode
	@echo "$(GREEN)Starting development environment...$(NC)"
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml up -d
	@echo "$(GREEN)Waiting for services to be ready...$(NC)"
	@sleep 10
	$(MAKE) migrate
	@echo "$(GREEN)Development environment ready!$(NC)"
	@echo "  LMS:        http://lms.axel.localhost"
	@echo "  Studio:     http://studio.axel.localhost"
	@echo "  Admin:      http://admin.axel.localhost"
	@echo "  API Docs:   http://api.axel.localhost/docs"
	@echo "  Grafana:    http://grafana.axel.localhost"
	@echo "  MinIO:      http://minio.axel.localhost"

dev-rust: ## Run Rust services in development mode
	cd $(RUST_DIR) && cargo run --bin auth-service &
	cd $(RUST_DIR) && cargo run --bin realtime-service &
	cd $(RUST_DIR) && cargo run --bin search-service &
	cd $(RUST_DIR) && cargo run --bin media-service &
	cd $(RUST_DIR) && cargo run --bin scheduler-service &

dev-python: ## Run Python services in development mode
	cd $(PYTHON_DIR) && pdm run uvicorn ai_service.main:app --reload --host 0.0.0.0 --port 8001 &
	cd $(PYTHON_DIR) && pdm run uvicorn analytics_service.main:app --reload --host 0.0.0.0 --port 8002 &
	cd $(PYTHON_DIR) && pdm run uvicorn grading_service.main:app --reload --host 0.0.0.0 --port 8003 &
	cd $(PYTHON_DIR) && pdm run uvicorn recommendation_service.main:app --reload --host 0.0.0.0 --port 8004 &
	cd $(PYTHON_DIR) && pdm run uvicorn nlp_service.main:app --reload --host 0.0.0.0 --port 8005 &

dev-php: ## Run PHP services in development mode
	cd $(PHP_DIR)/web-app && php artisan octane:start --server=swoole --host=0.0.0.0 --port=8000 --watch &
	cd $(PHP_DIR)/admin-panel && php artisan octane:start --server=swoole --host=0.0.0.0 --port=8001 --watch &

dev-infra: ## Start only infrastructure (DB, Redis, NATS, MinIO, etc.)
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml up -d postgres redis nats minio meilisearch jaeger

dev-logs: ## Follow logs from all services
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml logs -f

dev-stop: ## Stop development environment
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml down

dev-clean: ## Clean development environment (removes volumes)
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml down -v

# =============================================================================
# Build
# =============================================================================

build: build-rust build-python build-php build-proto ## Build all services

build-rust: ## Build Rust workspace
	@echo "$(GREEN)Building Rust workspace...$(NC)"
	cd $(RUST_DIR) && cargo build --workspace --release

build-python: ## Build Python workspace
	@echo "$(GREEN)Building Python workspace...$(NC)"
	cd $(PYTHON_DIR) && pdm sync --prod

build-php: ## Build PHP workspace
	@echo "$(GREEN)Building PHP workspace...$(NC)"
	cd $(PHP_DIR) && composer install --no-dev --optimize-autoloader --no-interaction

build-proto: ## Generate gRPC code from proto definitions
	@echo "$(GREEN)Generating gRPC code...$(NC)"
	cd $(RUST_DIR)/proto && cargo build
	# Python
	cd $(PYTHON_DIR) && python -m grpc_tools.protoc \
		-I$(CONTRACTS_DIR)/proto \
		--python_out=. \
		--grpc_python_out=. \
		$(CONTRACTS_DIR)/proto/apexos/**/v1/*.proto
	# PHP
	cd $(PHP_DIR) && protoc \
		-I$(CONTRACTS_DIR)/proto \
		--php_out=. \
		--grpc_out=. \
		--plugin=protoc-gen-grpc=/usr/local/bin/grpc_php_plugin \
		$(CONTRACTS_DIR)/proto/apexos/**/v1/*.proto

# =============================================================================
# Testing
# =============================================================================

test: test-rust test-python test-php test-contracts test-integration ## Run all tests

test-rust: ## Run Rust tests
	@echo "$(GREEN)Running Rust tests...$(NC)"
	cd $(RUST_DIR) && cargo test --workspace --all-targets

test-rust-unit: ## Run Rust unit tests only
	cd $(RUST_DIR) && cargo test --workspace --lib

test-rust-integration: ## Run Rust integration tests
	cd $(RUST_DIR) && cargo test --workspace --test integration

test-python: ## Run Python tests
	@echo "$(GREEN)Running Python tests...$(NC)"
	cd $(PYTHON_DIR) && pdm run pytest -xvs

test-python-unit: ## Run Python unit tests
	cd $(PYTHON_DIR) && pdm run pytest -xvs -m unit

test-python-integration: ## Run Python integration tests
	cd $(PYTHON_DIR) && pdm run pytest -xvs -m integration

test-php: ## Run PHP tests
	@echo "$(GREEN)Running PHP tests...$(NC)"
	cd $(PHP_DIR) && ./vendor/bin/pest --parallel

test-php-unit: ## Run PHP unit tests
	cd $(PHP_DIR) && ./vendor/bin/pest --filter=Unit

test-php-integration: ## Run PHP integration tests
	cd $(PHP_DIR) && ./vendor/bin/pest --filter=Integration

test-contracts: ## Test contract compatibility
	@echo "$(GREEN)Testing contract compatibility...$(NC)"
	# Run contract tests with Pact or similar
	cd tests/contracts && ./run_contract_tests.sh

test-integration: ## Run cross-service integration tests
	@echo "$(GREEN)Running integration tests...$(NC)"
	cd tests/integration && ./run_integration_tests.sh

test-e2e: ## Run end-to-end tests
	@echo "$(GREEN)Running E2E tests...$(NC)"
	cd tests/e2e && ./run_e2e_tests.sh

test-performance: ## Run performance tests
	@echo "$(GREEN)Running performance tests...$(NC)"
	cd tests/performance && ./run_perf_tests.sh

test-coverage: ## Generate test coverage reports
	@echo "$(GREEN)Generating coverage reports...$(NC)"
	cd $(RUST_DIR) && cargo llvm-cov --workspace --html --output-dir ../target/coverage/rust
	cd $(PYTHON_DIR) && pdm run pytest --cov --cov-report=html:../target/coverage/python
	cd $(PHP_DIR) && ./vendor/bin/pest --coverage --coverage-html=../target/coverage/php

# =============================================================================
# Linting & Formatting
# =============================================================================

lint: lint-rust lint-python lint-php lint-proto ## Run all linters

lint-rust: ## Lint Rust code
	@echo "$(GREEN)Linting Rust...$(NC)"
	cd $(RUST_DIR) && cargo clippy --workspace --all-targets -- -D warnings
	cd $(RUST_DIR) && cargo fmt --check

lint-python: ## Lint Python code
	@echo "$(GREEN)Linting Python...$(NC)"
	cd $(PYTHON_DIR) && pdm run ruff check .
	cd $(PYTHON_DIR) && pdm run mypy .

lint-php: ## Lint PHP code
	@echo "$(GREEN)Linting PHP...$(NC)"
	cd $(PHP_DIR) && ./vendor/bin/phpstan analyse --memory-limit=2G
	cd $(PHP_DIR) && ./vendor/bin/psalm --no-cache
	cd $(PHP_DIR) && ./vendor/bin/pint --test

lint-proto: ## Lint proto files
	@echo "$(GREEN)Linting proto files...$(NC)"
	buf lint $(CONTRACTS_DIR)/proto

fmt: fmt-rust fmt-python fmt-php ## Format all code

fmt-rust: ## Format Rust code
	cd $(RUST_DIR) && cargo fmt --all

fmt-python: ## Format Python code
	cd $(PYTHON_DIR) && pdm run ruff format .

fmt-php: ## Format PHP code
	cd $(PHP_DIR) && ./vendor/bin/pint

fmt-check: fmt-check-rust fmt-check-python fmt-check-php ## Check formatting without changes

fmt-check-rust:
	cd $(RUST_DIR) && cargo fmt --check

fmt-check-python:
	cd $(PYTHON_DIR) && pdm run ruff format --check .

fmt-check-php:
	cd $(PHP_DIR) && ./vendor/bin/pint --test

# =============================================================================
# Database
# =============================================================================

migrate: migrate-rust migrate-python migrate-php ## Run all migrations

migrate-rust: ## Run Rust migrations (sqlx)
	@echo "$(GREEN)Running Rust migrations...$(NC)"
	cd $(RUST_DIR) && cargo sqlx migrate run --source migrations

migrate-python: ## Run Python migrations (alembic)
	@echo "$(GREEN)Running Python migrations...$(NC)"
	cd $(PYTHON_DIR) && pdm run alembic upgrade head

migrate-php: ## Run PHP migrations (Laravel)
	@echo "$(GREEN)Running PHP migrations...$(NC)"
	cd $(PHP_DIR)/web-app && php artisan migrate --force
	cd $(PHP_DIR)/admin-panel && php artisan migrate --force

migrate-create-rust: ## Create new Rust migration
	@read -p "Migration name: " name; \
	cd $(RUST_DIR) && cargo sqlx migrate add $$name --source migrations

migrate-create-python: ## Create new Python migration
	@read -p "Migration name: " name; \
	cd $(PYTHON_DIR) && pdm run alembic revision --autogenerate -m "$$name"

migrate-create-php: ## Create new PHP migration
	@read -p "Migration name: " name; \
	cd $(PHP_DIR)/web-app && php artisan make:migration $$name

migrate-rollback: ## Rollback last migration (PHP)
	cd $(PHP_DIR)/web-app && php artisan migrate:rollback --force
	cd $(PHP_DIR)/admin-panel && php artisan migrate:rollback --force

migrate-fresh: ## Fresh migrate with seeding (PHP)
	cd $(PHP_DIR)/web-app && php artisan migrate:fresh --seed --force
	cd $(PHP_DIR)/admin-panel && php artisan migrate:fresh --seed --force

seed: seed-rust seed-python seed-php ## Seed all databases

seed-rust: ## Seed Rust database
	cd $(RUST_DIR) && cargo run --bin seed

seed-python: ## Seed Python database
	cd $(PYTHON_DIR) && pdm run seed

seed-php: ## Seed PHP database
	cd $(PHP_DIR)/web-app && php artisan db:seed --force
	cd $(PHP_DIR)/admin-panel && php artisan db:seed --force

# =============================================================================
# Docker
# =============================================================================

docker-build: ## Build all Docker images
	@echo "$(GREEN)Building Docker images...$(NC)"
	docker build -t $(DOCKER_REGISTRY)/axel-auth-service:$(DOCKER_TAG) -f $(RUST_DIR)/auth-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-realtime-service:$(DOCKER_TAG) -f $(RUST_DIR)/realtime-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-search-service:$(DOCKER_TAG) -f $(RUST_DIR)/search-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-media-service:$(DOCKER_TAG) -f $(RUST_DIR)/media-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-scheduler-service:$(DOCKER_TAG) -f $(RUST_DIR)/scheduler-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-ai-service:$(DOCKER_TAG) -f $(PYTHON_DIR)/ai-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-analytics-service:$(DOCKER_TAG) -f $(PYTHON_DIR)/analytics-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-grading-service:$(DOCKER_TAG) -f $(PYTHON_DIR)/grading-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-recommendation-service:$(DOCKER_TAG) -f $(PYTHON_DIR)/recommendation-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-nlp-service:$(DOCKER_TAG) -f $(PYTHON_DIR)/nlp-service/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-web-app:$(DOCKER_TAG) -f $(PHP_DIR)/web-app/Dockerfile .
	docker build -t $(DOCKER_REGISTRY)/axel-admin-panel:$(DOCKER_TAG) -f $(PHP_DIR)/admin-panel/Dockerfile .

docker-push: docker-build ## Push Docker images to registry
	@echo "$(GREEN)Pushing Docker images...$(NC)"
	docker push $(DOCKER_REGISTRY)/axel-auth-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-realtime-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-search-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-media-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-scheduler-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-ai-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-analytics-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-grading-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-recommendation-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-nlp-service:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-web-app:$(DOCKER_TAG)
	docker push $(DOCKER_REGISTRY)/axel-admin-panel:$(DOCKER_TAG)

docker-up: ## Start production Docker stack
	docker compose -f $(INFRA_DIR)/docker/compose.prod.yml up -d

docker-down: ## Stop production Docker stack
	docker compose -f $(INFRA_DIR)/docker/compose.prod.yml down

docker-logs: ## Follow production logs
	docker compose -f $(INFRA_DIR)/docker/compose.prod.yml logs -f

# =============================================================================
# Kubernetes
# =============================================================================

k8s-deploy: ## Deploy to Kubernetes
	@echo "$(GREEN)Deploying to Kubernetes...$(NC)"
	helm upgrade --install axel $(DIST_DIR)/helm-charts/axel \
		--namespace axel \
		--create-namespace \
		--set image.tag=$(DOCKER_TAG) \
		--wait --timeout 10m

k8s-upgrade: ## Upgrade Kubernetes deployment
	helm upgrade axel $(DIST_DIR)/helm-charts/axel \
		--namespace axel \
		--set image.tag=$(DOCKER_TAG) \
		--wait --timeout 10m

k8s-rollback: ## Rollback Kubernetes deployment
	helm rollback axel --namespace axel

k8s-status: ## Check Kubernetes deployment status
	kubectl get pods -n axel -l app.kubernetes.io/instance=axel
	kubectl get svc -n axel -l app.kubernetes.io/instance=axel

k8s-logs: ## View Kubernetes logs
	kubectl logs -n axel -l app.kubernetes.io/instance=axel -f --tail=100

# =============================================================================
# Distribution
# =============================================================================

package: package-installer package-helm package-airgap ## Create all distribution packages

package-installer: ## Create offline installer
	@echo "$(GREEN)Creating offline installer...$(NC)"
	$(DIST_DIR)/installer/create_installer.sh $(VERSION)

package-helm: ## Package Helm charts
	@echo "$(GREEN)Packaging Helm charts...$(NC)"
	helm package $(DIST_DIR)/helm-charts/axel -d $(DIST_DIR)/helm-charts/packaged

package-airgap: ## Create air-gapped deployment bundle
	@echo "$(GREEN)Creating air-gapped bundle...$(NC)"
	$(DIST_DIR)/airgap/create_bundle.sh $(VERSION)

# =============================================================================
# Release
# =============================================================================

release: ## Create a new release
	@echo "$(GREEN)Creating release $(VERSION)...$(NC)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	git push origin v$(VERSION)
	$(MAKE) docker-push
	$(MAKE) package
	@echo "$(GREEN)Release $(VERSION) created!$(NC)"

release-dry-run: ## Dry run release process
	@echo "$(YELLOW)Dry run: Would create release $(VERSION)$(NC)"
	@echo "  - Tag: v$(VERSION)"
	@echo "  - Docker images: $(DOCKER_REGISTRY)/axel-*:$(DOCKER_TAG)"
	@echo "  - Packages: installer, helm, airgap"

# =============================================================================
# Utilities
# =============================================================================

secrets: ## Generate development secrets
	@echo "$(GREEN)Generating development secrets...$(NC)"
	@mkdir -p $(INFRA_DIR)/docker/secrets
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/jwt_secret.txt
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/db_password.txt
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/redis_password.txt
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/minio_access_key.txt
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/minio_secret_key.txt
	@openssl rand -base64 32 > $(INFRA_DIR)/docker/secrets/webhook_secret.txt
	@echo "$(GREEN)Secrets generated in $(INFRA_DIR)/docker/secrets/$(NC)"

health: ## Check health of all services
	@echo "$(GREEN)Checking service health...$(NC)"
	@curl -sf http://localhost:8000/health/live || echo "$(RED)auth-service: DOWN$(NC)"
	@curl -sf http://localhost:8001/health/live || echo "$(RED)realtime-service: DOWN$(NC)"
	@curl -sf http://localhost:8002/health/live || echo "$(RED)search-service: DOWN$(NC)"
	@curl -sf http://localhost:8003/health/live || echo "$(RED)media-service: DOWN$(NC)"
	@curl -sf http://localhost:8004/health/live || echo "$(RED)scheduler-service: DOWN$(NC)"
	@curl -sf http://localhost:8005/health/live || echo "$(RED)ai-service: DOWN$(NC)"
	@curl -sf http://localhost:8006/health/live || echo "$(RED)analytics-service: DOWN$(NC)"
	@curl -sf http://localhost:8007/health/live || echo "$(RED)grading-service: DOWN$(NC)"
	@curl -sf http://localhost:8008/health/live || echo "$(RED)recommendation-service: DOWN$(NC)"
	@curl -sf http://localhost:8009/health/live || echo "$(RED)nlp-service: DOWN$(NC)"
	@curl -sf http://localhost:8000/health/live || echo "$(RED)web-app: DOWN$(NC)"
	@curl -sf http://localhost:8001/health/live || echo "$(RED)admin-panel: DOWN$(NC)"

benchmark: ## Run benchmarks
	@echo "$(GREEN)Running benchmarks...$(NC)"
	cd $(RUST_DIR) && cargo bench --workspace
	cd $(PYTHON_DIR) && pdm run pytest --benchmark-only

docs: ## Generate documentation
	@echo "$(GREEN)Generating documentation...$(NC)"
	cd $(RUST_DIR) && cargo doc --workspace --no-deps --document-private-items
	cd $(PYTHON_DIR) && pdm run mkdocs build
	cd $(PHP_DIR) && ./vendor/bin/phpdoc run

docs-serve: ## Serve documentation locally
	cd $(RUST_DIR) && cargo doc --workspace --no-deps --document-private-items --open
	cd $(PYTHON_DIR) && pdm run mkdocs serve

clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning build artifacts...$(NC)"
	cd $(RUST_DIR) && cargo clean
	cd $(PYTHON_DIR) && pdm run python -m pip cache purge
	cd $(PHP_DIR) && rm -rf vendor
	rm -rf target
	rm -rf $(DIST_DIR)/helm-charts/packaged
	rm -rf $(DIST_DIR)/installer/output
	rm -rf $(DIST_DIR)/airgap/output

clean-all: clean ## Clean everything including dependencies
	rm -rf $(RUST_DIR)/target
	rm -rf $(PYTHON_DIR)/.venv
	rm -rf $(PHP_DIR)/vendor
	docker system prune -f

# =============================================================================
# CI/CD Helpers
# =============================================================================

ci-lint: lint ## CI lint target

ci-test: test ## CI test target

ci-build: build ## CI build target

ci-security: ## Security audit
	@echo "$(GREEN)Running security audit...$(NC)"
	cd $(RUST_DIR) && cargo audit
	cd $(PYTHON_DIR) && pdm run pip-audit
	cd $(PHP_DIR) && ./vendor/bin/security-checker security:check composer.lock

ci-dependencies: ## Check for outdated dependencies
	@echo "$(GREEN)Checking dependencies...$(NC)"
	cd $(RUST_DIR) && cargo outdated
	cd $(PYTHON_DIR) && pdm list --outdated
	cd $(PHP_DIR) && composer outdated

# =============================================================================
# Development Helpers
# =============================================================================

proto-check: ## Check proto files for breaking changes
	@echo "$(GREEN)Checking proto compatibility...$(NC)"
	buf breaking $(CONTRACTS_DIR)/proto --against '.git#branch=main'

generate-ide-helpers: ## Generate IDE helpers for PHP
	cd $(PHP_DIR)/web-app && php artisan ide-helper:generate
	cd $(PHP_DIR)/web-app && php artisan ide-helper:meta
	cd $(PHP_DIR)/web-app && php artisan ide-helper:models -N

reset-db: ## Reset all databases (DANGEROUS)
	@echo "$(RED)WARNING: This will destroy all data!$(NC)"
	@read -p "Are you sure? (yes/no): " confirm && [ "$$confirm" = "yes" ] || exit 1
	docker compose -f $(INFRA_DIR)/docker/compose.dev.yml down -v
	$(MAKE) dev-infra
	$(MAKE) migrate
	$(MAKE) seed

# =============================================================================
# Version Info
# =============================================================================

version: ## Print version information
	@echo "Project: $(PROJECT_NAME)"
	@echo "Version: $(VERSION)"
	@echo "Build Date: $(BUILD_DATE)"
	@echo "Git Commit: $(GIT_COMMIT)"
	@echo "Rust: $(RUST_VERSION)"
	@echo "Python: $(PYTHON_VERSION)"
	@echo "PHP: $(PHP_VERSION)"
	@echo "Node: $(NODE_VERSION)"