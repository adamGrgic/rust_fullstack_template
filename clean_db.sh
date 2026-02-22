#!/bin/bash

# Atom Platform Database Cleanup Script
# Removes the PostgreSQL database and user
#
# NOTE: Run this script as your regular user (NOT with sudo)
# The script will use sudo internally when needed.

set -e

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "⚠️  Please run this script as your regular user (without sudo)"
    echo "   The script will prompt for sudo when needed."
    echo ""
    echo "   Run: ./clean_db.sh"
    exit 1
fi

echo "🧹 Atom Platform Database Cleanup"
echo "=================================="
echo ""
echo "⚠️  WARNING: This will DELETE the following:"
echo "   - Database: atom"
echo "   - User: atom"
echo "   - All data in the database"
echo ""

# Configuration
DB_NAME="atom"
DB_USER="atom"

# Ask for confirmation
read -p "Are you sure you want to continue? (yes/no): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    echo "❌ Cleanup cancelled"
    exit 0
fi

echo ""

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL client (psql) is not installed."
    exit 1
fi

# Detect OS and set appropriate connection method
detect_postgres_connection() {
    # Try connecting as postgres user (Linux/WSL typical)
    if sudo -u postgres psql -c '\q' 2>/dev/null; then
        echo "✅ PostgreSQL connection detected"
        PSQL_CMD="sudo -u postgres psql"
        return 0
    fi
    
    # Try connecting as current user (macOS typical)
    if psql postgres -c '\q' 2>/dev/null; then
        echo "✅ PostgreSQL connection detected"
        PSQL_CMD="psql postgres"
        return 0
    fi
    
    # Try connecting as postgres with host (may need password)
    if psql -U postgres -h localhost -c '\q' 2>/dev/null; then
        echo "✅ PostgreSQL connection detected"
        PSQL_CMD="psql -U postgres -h localhost"
        return 0
    fi
    
    echo "❌ Cannot connect to PostgreSQL."
    echo "Please ensure PostgreSQL is running."
    exit 1
}

# Detect connection method
detect_postgres_connection

# Terminate existing connections to the database
echo ""
echo "Terminating active connections to '$DB_NAME'..."
$PSQL_CMD -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname='$DB_NAME' AND pid <> pg_backend_pid();" 2>/dev/null || true
echo "✅ Connections terminated"

# Drop database
echo ""
echo "Checking if database '$DB_NAME' exists..."
DB_EXISTS=$($PSQL_CMD -tAc "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'" 2>/dev/null)

if [ "$DB_EXISTS" = "1" ]; then
    echo "🗑️  Dropping database '$DB_NAME'..."
    $PSQL_CMD -c "DROP DATABASE $DB_NAME;" 2>/dev/null
    echo "✅ Database '$DB_NAME' dropped"
else
    echo "ℹ️  Database '$DB_NAME' does not exist (already removed)"
fi

# Drop user
echo ""
echo "Checking if user '$DB_USER' exists..."
USER_EXISTS=$($PSQL_CMD -tAc "SELECT 1 FROM pg_roles WHERE rolname='$DB_USER'" 2>/dev/null)

if [ "$USER_EXISTS" = "1" ]; then
    echo "🗑️  Dropping user '$DB_USER'..."
    $PSQL_CMD -c "DROP USER $DB_USER;" 2>/dev/null
    echo "✅ User '$DB_USER' dropped"
else
    echo "ℹ️  User '$DB_USER' does not exist (already removed)"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Cleanup complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "The following have been removed:"
echo "  ✓ Database: $DB_NAME"
echo "  ✓ User: $DB_USER"
echo "  ✓ All data associated with the database"
echo ""
echo "NOTE: .env file unchanged - DATABASE_URL still configured"
echo ""
echo "To recreate the database, run:"
echo "  ./setup_db.sh"
echo ""
