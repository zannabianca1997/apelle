# Makefile for generating Docker Compose files from Jsonnet templates

# Default values for tools
JSONNET ?= rsjsonnet
PYTHON ?= python3

# Get the script directory (where this Makefile is located)
SCRIPT_DIR := $(shell cd -- "$( dirname -- "$(MAKEFILE_LIST)" )" &> /dev/null && pwd)

# Environment targets
ENVIRONS := dev build prod

# Directory paths
SRC_DIR := $(SCRIPT_DIR)/src/artifacts
OUTPUT_DIR := $(SCRIPT_DIR)/composes
PROJ_DIR := $(SCRIPT_DIR)/..

# Dependencies
DEPLOY_JSON := $(PROJ_DIR)/deploy.json

SONNETS := $(shell find $(SCRIPT_DIR)/src/ -type f -name '*.libsonnet')

MAKEFLAGS += -rR

# Default target
.PHONY: all
all: $(ENVIRONS)

# Generate compose files for all environments
.PHONY: $(ENVIRONS)
$(ENVIRONS): % : $(OUTPUT_DIR)/compose.%.yml


$(OUTPUT_DIR)/compose.%.yml: $(SRC_DIR)/compose.%.jsonnet $(SONNETS) $(DEPLOY_JSON)
	@echo "🔨 Building $* compose"
	@mkdir -p $(OUTPUT_DIR)
	$(JSONNET) --ext-str projDir=../.. -J "$(PROJ_DIR)" "$<" | $(PYTHON) -c 'import sys, yaml, json; j=json.loads(sys.stdin.read()); print(yaml.safe_dump(j))' > "$@"

# Clean generated files
.PHONY: clean
clean:
	@echo "🧹 Cleaning generated compose files"
	@rm -rf $(OUTPUT_DIR)/*.yml

# Help target
.PHONY: help
help:
	@echo "❓ Available targets:"
	@echo "  all     - Generate compose files for all environments ($(ENVIRONS))"
	@echo "  dev     - Generate development compose file"
	@echo "  build   - Generate build compose file"
	@echo "  prod    - Generate production compose file"
	@echo "  clean   - Remove all generated compose files"
	@echo "  help    - Show this help message"
	@echo ""
	@echo "⚙️ Environment variables:"
	@echo "  JSONNET - Jsonnet executable (default: rsjsonnet)"
	@echo "  PYTHON  - Python executable (default: python3)"