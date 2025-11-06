#!/bin/env bash

# Prepare script for Apelle project
# This script prepares the repository for development

# Command variables (inherited from Makefile)
DOCKER="${DOCKER:-docker}"
CARGO="${CARGO:-cargo}"
NVM_SH="${NVM_SH:-$HOME/.nvm/nvm.sh}"
NVM="${NVM:-nvm}"
JSONNET="${JSONNET:-rsjsonnet}"
PYTHON="${PYTHON:-python3}"

PROJ_DIR="${PROJ_DIR:-.}"

set -uo pipefail

echo "⚙️ Preparing the repo for development..."
echo
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
    nvm_version=$($NVM -v 2>/dev/null)
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
echo
echo "🔧 Setting up environment variables..."

# Check if .env already exists
skip_envs=false

# Initialize empty variables
YOUTUBE_API_KEY="${YOUTUBE_API_KEY:-}"
DEPLOY_DOCKER_HOST="${DEPLOY_DOCKER_HOST:-}"
DEPLOY_POSTGRES_PASSWORD="${DEPLOY_POSTGRES_PASSWORD:-}"

pushd "${PROJ_DIR}"
    if [ -f ".env" ]; then
        echo "⚠️  .env file already exists."
        read -p "Do you want to overwrite it? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "⏭️  Skipping environment variables setup."
            skip_envs=true
        fi

        set -a  # Export all variables
        source .env
        set +a
    fi

    if [ "$skip_envs" = false ]; then
        # Read environment variables from .env.example
        echo "📝 Please provide values for the following environment variables:"
        echo ""


        # Prompt for YouTube API Key
        echo "🎬 YouTube API Key (required for songs-provider-youtube)"
        if [ -n "$YOUTUBE_API_KEY" ]; then
            read -s -p "Enter your YouTube API Key [${YOUTUBE_API_KEY:0:10}...] (or press Enter to keep current): " input
            if [ -n "$input" ]; then
                YOUTUBE_API_KEY="$input"
            fi
        else
            read -s -p "Enter your YouTube API Key (or press Enter to skip): " YOUTUBE_API_KEY
        fi
        echo

        # Prompt for Deploy Docker Host
        echo "🐳 Deploy Docker Host (optional, for deployment)"
        if [ -n "$DEPLOY_DOCKER_HOST" ]; then
            read -p "Enter your deploy Docker host [$DEPLOY_DOCKER_HOST] (or press Enter to keep current): " input
            if [ -n "$input" ]; then
                DEPLOY_DOCKER_HOST="$input"
            fi
        else
            read -p "Enter your deploy Docker host (or press Enter to skip): " DEPLOY_DOCKER_HOST
        fi

        # Prompt for Deploy Postgres Password
        echo "🐘 Deploy Postgres Password (optional, for deployment)"
        if [ -n "$DEPLOY_POSTGRES_PASSWORD" ]; then
            read -s -p "Enter your deploy Postgres password [****] (or press Enter to keep current): " input
            if [ -n "$input" ]; then
                DEPLOY_POSTGRES_PASSWORD="$input"
            fi
        else
            read -s -p "Enter your deploy Postgres password (or press Enter to skip): " DEPLOY_POSTGRES_PASSWORD
        fi
        echo

        # Create .env file
        echo "📄 Creating .env file..."
        cat > .env << EOF
YOUTUBE_API_KEY=${YOUTUBE_API_KEY}
DEPLOY_DOCKER_HOST=${DEPLOY_DOCKER_HOST}
DEPLOY_POSTGRES_PASSWORD=${DEPLOY_POSTGRES_PASSWORD}
EOF

        echo "✅ .env file created successfully!"
    fi

popd

echo
echo "📦 Installing frontend dependencies..."
pushd "${PROJ_DIR}/web-ui"
    . "$NVM_SH"
    $NVM use
    npm ci
    npm run prepare
    if [ $? -eq 0 ]; then
        echo "✅ Frontend dependencies installed successfully!"
    else
        echo "❌ Error: Failed to install frontend dependencies."
        exit 1
    fi   
popd

echo
echo "🔧 Running initial cargo check to warm up caches..."

# Run cargo check to warm up caches and speed up future checks
pushd "${PROJ_DIR}"
    $CARGO check
    if [ $? -eq 0 ]; then
        echo "✅ Initial cargo check completed successfully!"
    else
        echo "⚠️  Cargo check completed with warnings/errors."
    fi
popd

echo
echo "✅ Repo is ready to develop!"
