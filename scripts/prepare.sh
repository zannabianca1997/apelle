#!/bin/env bash

# Prepare script for Apelle project
# This script prepares the repository for development

# Command variables (inherited from Makefile)
DOCKER="${DOCKER:-docker}"
CARGO="${CARGO:-cargo}"
NVM_SH="${NVM_SH:-$HOME/.nvm/nvm.sh}"
JSONNET="${JSONNET:-rsjsonnet}"
PYTHON="${PYTHON:-python3}"

set -e

echo "⚙️ Preparing the repo for development..."

echo "🔍 Checking developement tools..."

missing_tools=false

# Check Docker
docker_version=$($DOCKER -v)
if [ $? -eq 0 ]; then
    echo "✅ Docker found: ${docker_version}"
else
    echo "❌ Error: Docker is not available. Please install Docker or set the DOCKER enviroment variable to the correct value."
    missing_tools=true
fi

# Check Cargo
cargo_version=$($CARGO --version)
if [ $? -eq 0 ]; then
    echo "✅ Cargo found: ${cargo_version}"

    # Check SQLx CLI
    sqlx_version=$($CARGO sqlx --version 2>/dev/null)
    if [ $? -eq 0 ]; then
        echo "✅ SQLx CLI found: ${sqlx_version}"
    else
        echo "❌ Error: SQLx CLI is not available. Please install it with 'cargo install sqlx-cli'"
        missing_tools=true
    fi
else
    echo "❌ Error: Cargo is not available. Please install Rust and Cargo or set the CARGO environment variable to the correct value."
    echo "⚠️ SQLx CLI is needed to work on this project. Please install it with 'cargo install sqlx-cli'"
    missing_tools=true
fi

# Check nvm
if [ -f "$NVM_SH" ]; then
    # Source NVM and check if it works
    . "$NVM_SH"
    nvm_version=$(nvm -v 2>/dev/null)
    if [ $? -eq 0 ]; then
        echo "✅ nvm found: ${nvm_version}"
    else
        echo "❌ Error: NVM_SH script found but NVM command failed. Please check your nvm installation."
        missing_tools=true
    fi
else
    echo "❌ Error: nvm is not available at ${NVM_SH}. Please install nvm or set the NVM_SH environment variable to the correct value."
    missing_tools=true
fi

# Check jsonnet
jsonnet_test=$($JSONNET -e "{}" 2>/dev/null)
if [ $? -eq 0 ]; then
    echo "✅ jsonnet found: ${JSONNET}"
else
    echo "❌ Error: jsonnet is not available. Please install rsjsonnet or set the JSONNET environment variable to the correct value."
    missing_tools=true
fi

# Check Python
python_version=$($PYTHON --version 2>&1)
if [ $? -eq 0 ]; then
    # Check if it's Python 3.x.x
    if echo "$python_version" | grep -q "^Python 3\."; then
        echo "✅ Python found: ${python_version}"
    else
        echo "❌ Error: Python 3.x.x is required, but found: ${python_version}"
        missing_tools=true
    fi
else
    echo "❌ Error: Python is not available. Please install Python 3.x.x or set the PYTHON environment variable to the correct value."
    missing_tools=true
fi

if [ "$missing_tools" = true ]; then
    echo "⚠️  Some required tools are missing. Please install them and try again."
    exit 1
fi

echo "✅ All required tools are available!"

# Environment variables setup
echo ""
echo "🔧 Setting up environment variables..."

# Check if .env already exists
skip_envs=false
if [ -f ".env" ]; then
    echo "⚠️  .env file already exists."
    read -p "Do you want to overwrite it? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "⏭️  Skipping environment variables setup."
        skip_envs=true
    fi
fi

if [ "$skip_envs" = false ]; then
    # Read environment variables from .env.example
    echo "📝 Please provide values for the following environment variables:"
    echo ""

    # Initialize empty variables
    YOUTUBE_API_KEY=""

    # Prompt for YouTube API Key
    echo "🎬 YouTube API Key (required for songs-provider-youtube)"
    read -s -p "Enter your YouTube API Key (or press Enter to skip): " YOUTUBE_API_KEY
    echo

    # Create .env file
    echo "📄 Creating .env file..."
    cat > .env << EOF
YOUTUBE_API_KEY=${YOUTUBE_API_KEY}
EOF

    echo "✅ .env file created successfully!"
fi

echo "✅ Repo is ready to develop!"