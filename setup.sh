#!/bin/bash

# Atom Platform Setup Script
# This script helps set up the development environment

set -e

echo "🚀 Atom Platform Setup"
echo "====================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust is installed"

# Check if PostgreSQL is available
if command -v psql &> /dev/null; then
    echo "✅ PostgreSQL client is installed"
else
    echo "⚠️  PostgreSQL client not found. Please install PostgreSQL."
    echo "   Linux/WSL: sudo apt install postgresql postgresql-contrib"
    echo "   macOS: brew install postgresql@16"
fi

# Check for trunk
if ! command -v trunk &> /dev/null; then
    echo "📦 Installing Trunk..."
    cargo install trunk
else
    echo "✅ Trunk is installed"
fi

# Check for wasm32 target
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "✅ wasm32-unknown-unknown target is installed"
else
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Check for Tailwind CSS CLI
if [ ! -f "client/tailwindcss" ]; then
    echo "📦 Downloading Tailwind CSS CLI..."
    curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
    chmod +x tailwindcss-linux-x64
    mv tailwindcss-linux-x64 client/tailwindcss
    echo "✅ Tailwind CSS CLI installed"
else
    echo "✅ Tailwind CSS CLI is installed"
fi

# Setup environment file
if [ ! -f "platform/api/.env" ]; then
    echo "📝 Creating .env file..."
    cp platform/api/env.example platform/api/.env
    echo "✅ Created platform/api/.env (please review and update if needed)"
else
    echo "✅ .env file already exists"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Setup PostgreSQL database and user:"
echo "   ./setup_db.sh"
echo ""
echo "2. Run database migrations:"
echo "   cd platform/api && cargo sqlx migrate run"
echo ""
echo "3. Start the full development environment:"
echo "   make dev"
echo ""
echo "   OR run components separately:"
echo "   - Terminal 1: cd platform/api && cargo run"
echo "   - Terminal 2: cd client && trunk serve"
echo ""
echo "4. Open your browser to http://localhost:8081"
echo ""
echo "For more information, see docs/SETUP.md"
echo ""

