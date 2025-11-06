# Makefile for Apelle project

MAKEFLAGS += -rR

.PHONY: default
default: up

# Development targets
.PHONY: up up-detach down build push _up-common
_up-common: .env
	@echo "Generating development compose file..."
	$(MAKE) -C docker dev
	@cp .env docker/composes/.env
	@echo "Starting development environment$(if $(filter -d,$(COMPOSE_ARGS)), in detached mode,)..."
	docker compose -f docker/composes/compose.dev.yml up --build $(COMPOSE_ARGS)

up: COMPOSE_ARGS :=
up: _up-common

up-detach: COMPOSE_ARGS := -d
up-detach: _up-common

down:
	@echo "Stopping development environment..."
	docker compose -f docker/composes/compose.dev.yml down

build:
	@echo "Generating build compose file..."
	$(MAKE) -C docker build
	@cp .env docker/composes/.env
	@echo "Building services..."
	docker compose -f docker/composes/compose.build.yml build

push: build
	@echo "Pushing built images..."
	docker compose -f docker/composes/compose.build.yml push

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  up        - Start development environment"
	@echo "  up-detach - Start development environment in detached mode"
	@echo "  down      - Stop development environment"
	@echo "  build     - Build services using build compose file"
	@echo "  push      - Build and push services using build compose file"
	@echo "  help    - Show this help message"