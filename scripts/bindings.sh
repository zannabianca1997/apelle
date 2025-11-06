#!/bin/env bash

# Bindings generation script for Apelle project
# This script generates API bindings (assumes dev environment is already running)

# Command variables (inherited from Makefile)
NVM_SH="${NVM_SH:-$HOME/.nvm/nvm.sh}"
NVM="${NVM:-nvm}"
NPM="${NPM:-npm}"

# Docker variables (inherited from docker/Makefile)
JSONNET="${JSONNET:-rsjsonnet}"
PYTHON="${PYTHON:-python3}"

set -euo pipefail

echo "🚀 Starting bindings generation..."

# Generate API documentation
echo "📚 Running api-docs generation..."
./api-docs/api-docs.sh

# Generate orval bindings
echo "🔗 Generating orval bindings..."
cd web-ui
. "$NVM_SH"
"$NVM" use
"$NPM" run orval
cd ..

echo "✅ Bindings generation completed!"