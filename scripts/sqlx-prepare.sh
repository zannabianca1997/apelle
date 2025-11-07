#!/bin/bash

# sqlx-prepare.sh - Prepare SQLx queries for the Apelle project
# This script starts the necessary services, runs sqlx prepare, and cleans up

set -euo pipefail

# Default values (can be overridden by environment)
POSTGRES_USER="${POSTGRES_USER:-apelle}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-apelle}"
POSTGRES_DB="${POSTGRES_DB:-apelle}"
DEV_DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"

# Command variables (can be overridden by environment)
DOCKER="${DOCKER:-docker}"
CARGO="${CARGO:-cargo}"


# Function to cleanup on exit
cleanup() {
    echo "🛑 Stopping services..."
    COMPOSE_PROJECT_NAME="apelle-sqlx-prepare" $DOCKER compose -f docker/composes/compose.dev.yml down || true
    $DOCKER volume rm apelle-sqlx-prepare-db-data || true
}

# Set up cleanup trap
trap cleanup EXIT

echo "🔧 Preparing SQLx queries..."

# Copy .env to docker/composes directory
cp .env docker/composes/.env

echo "🚀 Starting services for sqlx-prepare..."
COMPOSE_PROJECT_NAME="apelle-sqlx-prepare" $DOCKER compose -f docker/composes/compose.dev.yml up -d db migrator

echo "⏳ Waiting for migrator service to complete..."
COMPOSE_PROJECT_NAME="apelle-sqlx-prepare" $DOCKER compose -f docker/composes/compose.dev.yml wait migrator

echo "📝 Running cargo sqlx prepare on workspace..."
DATABASE_URL="$DEV_DATABASE_URL" $CARGO sqlx prepare --workspace

echo "✅ SQLx queries prepared successfully!"