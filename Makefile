# Makefile for Apelle project

include .env

MAKEFLAGS += -rR

# Command variables (can be overridden by environment)
DOCKER ?= docker
CARGO ?= cargo
NVM_SH ?= $(HOME)/.nvm/nvm.sh
NVM ?= nvm
NPM ?= npm

SHELL := /bin/bash

.PHONY: default
default: up

# Development targets
.PHONY: up up-detach down build push _up-common
_up-common: .env check composes-dev
	@cp .env docker/composes/.env
	@echo "🚀 Starting development environment$(if $(filter -d,$(COMPOSE_ARGS)), in detached mode,)..."
	$(DOCKER) compose -f docker/composes/compose.dev.yml up --build $(COMPOSE_ARGS)

up: COMPOSE_ARGS :=
up: _up-common

up-detach: COMPOSE_ARGS := -d
up-detach: _up-common

down:
	@echo "🛑 Stopping development environment..."
	$(DOCKER) compose -f docker/composes/compose.dev.yml down

build: check composes-build
	@cp .env docker/composes/.env
	@echo "🔨 Building services..."
	$(DOCKER) compose -f docker/composes/compose.build.yml build

push: build
	@echo "📤 Pushing built images..."
	$(DOCKER) compose -f docker/composes/compose.build.yml push

# Docker compose targets
COMPOSE_TARGETS := all dev build prod clean help
.PHONY: $(addprefix composes-,$(COMPOSE_TARGETS)) composes

composes: composes-all

$(addprefix composes-,$(COMPOSE_TARGETS)):
	@$(MAKE) -C docker $(patsubst composes-%,%,$@)

# Rust workspace targets
.PHONY: check format-rust format-web-ui format lint-rust lint-web-ui lint
check:
	@echo "🔍 Running cargo check on workspace..."
	SQLX_OFFLINE=true $(CARGO) check --workspace

format-rust:
	@echo "✨ Formatting Rust code..."
	$(CARGO) fmt

format-web-ui:
	@echo "🎨 Formatting web UI code..."
	@cd web-ui && . $(NVM_SH) && $(NVM) use && $(NPM) run format

format: format-rust format-web-ui
	@echo "✅ All code formatted successfully!"

lint-rust:
	@echo "🔧 Running cargo fix on Rust code..."
	$(CARGO) fix --allow-dirty --allow-staged

lint-web-ui:
	@echo "🔧 Running npm run lint:fix on web UI code..."
	@cd web-ui && . $(NVM_SH) && $(NVM) use && $(NPM) run lint:fix

lint: lint-rust lint-web-ui
	@echo "✅ All code linted successfully!"

# Bindings target
.PHONY: bindings
bindings: up-detach
	@NVM_SH="$(NVM_SH)" NVM="$(NVM)" NPM="$(NPM)" JSONNET="$(JSONNET)" PYTHON="$(PYTHON)" ./scripts/bindings.sh

# Prepare target
.PHONY: prepare
prepare:
	@DOCKER="$(DOCKER)" CARGO="$(CARGO)" NVM_SH="$(NVM_SH)" NVM="$(NVM)" NPM="$(NPM)" JSONNET="$(JSONNET)" PYTHON="$(PYTHON)" ./scripts/prepare.sh

# SQLx prepare target
.PHONY: sqlx-prepare
sqlx-prepare: .env composes-dev
	@DOCKER="$(DOCKER)" CARGO="$(CARGO)" ./scripts/sqlx-prepare.sh

# Deploy target
.PHONY: deploy
deploy: .env push composes-prod
	@cp .env docker/composes/.env
	@DOCKER="$(DOCKER)" DEPLOY_DOCKER_HOST="$(DEPLOY_DOCKER_HOST)" DEPLOY_POSTGRES_PASSWORD="$(DEPLOY_POSTGRES_PASSWORD)" ./scripts/deploy.sh

# Help target
.PHONY: help
help:
	@echo "❓ Available targets:"
	@echo "  prepare       - Prepare the repo for development"
	@echo "  up            - Start development environment"
	@echo "  up-detach     - Start development environment in detached mode"
	@echo "  down          - Stop development environment"
	@echo "  build         - Build services using build compose file"
	@echo "  push          - Build and push services using build compose file"
	@echo "  bindings      - Generate API bindings"
	@echo "  sqlx-prepare  - Prepare SQLx queries"
	@echo "  deploy        - Deploy the application"
	@echo "  check         - Run cargo check on the entire workspace"
	@echo "  format        - Format all code (Rust and web UI)"
	@echo "  format-rust   - Format Rust code"
	@echo "  format-web-ui - Format web UI"
	@echo "  lint          - Lint all code (Rust and web UI)"
	@echo "  lint-rust     - Run cargo fix on Rust code"
	@echo "  lint-web-ui   - Run npm run lint:fix on web UI"
	@echo "  composes-*    - Targets for the docker/Makefile"
	@echo "  help          - Show this help message"