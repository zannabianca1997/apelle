#!/bin/bash

# Deploy script for Apelle project
# This script handles deployment operations

# Command variables (inherited from Makefile)
DOCKER="${DOCKER:-docker}"

# Deploy variables (inherited from Makefile/environment)
DEPLOY_DOCKER_HOST="${DEPLOY_DOCKER_HOST:-}"
DEPLOY_POSTGRES_PASSWORD="${DEPLOY_POSTGRES_PASSWORD:-}"

set -euo pipefail

echo "🚀 Starting deployment process..."

if [ -z "$DEPLOY_DOCKER_HOST" ]; then
    echo "⚠️  DEPLOY_DOCKER_HOST not set"
    exit 1
else
    echo "🎯 Using remote Docker host: $DEPLOY_DOCKER_HOST"
fi

if [ -z "$DEPLOY_POSTGRES_PASSWORD" ]; then
    echo "⚠️  DEPLOY_POSTGRES_PASSWORD not set"
    exit 1
else
    echo "🔐 Using provided PostgreSQL password"
fi

echo "📥 Pulling containers on remote host..."
POSTGRES_PASSWORD="$DEPLOY_POSTGRES_PASSWORD" DOCKER_HOST="$DEPLOY_DOCKER_HOST" $DOCKER compose -f docker/composes/compose.prod.yml pull

echo "🚀 Starting containers on remote host..."
POSTGRES_PASSWORD="$DEPLOY_POSTGRES_PASSWORD" DOCKER_HOST="$DEPLOY_DOCKER_HOST" $DOCKER compose -f docker/composes/compose.prod.yml up -d

echo "✅ Deploy completed!"