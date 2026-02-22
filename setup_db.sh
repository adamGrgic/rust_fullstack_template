#!/bin/bash

# Atom Platform Database Setup Script
# Creates the PostgreSQL database and user if they don't exist
#
# NOTE: Run this script as your regular user (NOT with sudo)
# The script will use sudo internally when needed.

set -e

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "⚠️  Please run this script as your regular user (without sudo)"
    echo "   The script will prompt for sudo when needed."
    echo ""
    echo "   Run: ./setup_db.sh"
    exit 1
fi

echo "🗄️  Atom Platform Database Setup"
echo "================================"
echo ""

# Configuration
DB_NAME="atom"
DB_USER="atom"
DB_PASSWORD="atom"

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL client (psql) is not installed."
    echo "Please install PostgreSQL first:"
    echo "  - Linux/WSL: sudo apt install postgresql postgresql-contrib"
    echo "  - macOS: brew install postgresql@16"
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
    echo "Please ensure PostgreSQL is running:"
    echo "  - Linux/WSL: sudo service postgresql start"
    echo "  - macOS: brew services start postgresql@16"
    echo "  - Windows: Check Services for postgresql service"
    exit 1
}

# Detect connection method
detect_postgres_connection

# Check if database exists
echo ""
echo "Checking if database '$DB_NAME' exists..."
DB_EXISTS=$($PSQL_CMD -tAc "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'" 2>/dev/null)

if [ "$DB_EXISTS" = "1" ]; then
    echo "✅ Database '$DB_NAME' already exists"
else
    echo "📝 Creating database '$DB_NAME'..."
    $PSQL_CMD -c "CREATE DATABASE $DB_NAME;" 2>/dev/null
    echo "✅ Database '$DB_NAME' created successfully"
fi

# Check if user exists
echo ""
echo "Checking if user '$DB_USER' exists..."
USER_EXISTS=$($PSQL_CMD -tAc "SELECT 1 FROM pg_roles WHERE rolname='$DB_USER'" 2>/dev/null)

if [ "$USER_EXISTS" = "1" ]; then
    echo "✅ User '$DB_USER' already exists"
else
    echo "📝 Creating user '$DB_USER'..."
    $PSQL_CMD -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';" 2>/dev/null
    echo "✅ User '$DB_USER' created successfully"
fi

# Grant privileges
echo ""
echo "Setting up permissions..."
$PSQL_CMD -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;" 2>/dev/null
$PSQL_CMD -d $DB_NAME -c "GRANT ALL ON SCHEMA public TO $DB_USER;" 2>/dev/null
echo "✅ Permissions granted"

# Update .env file
echo ""
echo "Updating environment configuration..."
ENV_FILE="platform/api/.env"
DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME"

if [ -f "$ENV_FILE" ]; then
    # Update existing DATABASE_URL
    if grep -q "^DATABASE_URL=" "$ENV_FILE"; then
        # Use a different delimiter since URL contains slashes
        sed -i.bak "s|^DATABASE_URL=.*|DATABASE_URL=$DATABASE_URL|" "$ENV_FILE"
        rm -f "$ENV_FILE.bak"
        echo "✅ Updated DATABASE_URL in $ENV_FILE"
    else
        echo "DATABASE_URL=$DATABASE_URL" >> "$ENV_FILE"
        echo "✅ Added DATABASE_URL to $ENV_FILE"
    fi
else
    # Create new .env file
    cat > "$ENV_FILE" <<EOF
DATABASE_URL=$DATABASE_URL
PORT=8080
RUST_LOG=platform_api=debug,tower_http=debug
EOF
    echo "✅ Created $ENV_FILE"
fi

echo ""
echo "🎉 Database setup complete!"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "DATABASE CREDENTIALS (Development Only)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Database:    $DB_NAME"
echo "  Username:    $DB_USER"
echo "  Password:    $DB_PASSWORD"
echo "  Host:        localhost:5432"
echo "  URL:         $DATABASE_URL"
echo ""
echo "⚠️  IMPORTANT: These are DEVELOPMENT credentials only!"
echo "   Change these for production environments."
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TESTING THE DATABASE CONNECTION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "To login to PostgreSQL as the '$DB_USER' user:"
echo ""
echo "  psql -U $DB_USER -d $DB_NAME -h localhost"
echo ""
echo "  When prompted for password, enter: $DB_PASSWORD"
echo ""
echo "Once connected, you can run SQL commands:"
echo "  \\dt              # List all tables"
echo "  \\du              # List all users"
echo "  \\l               # List all databases"
echo "  SELECT * FROM todos;  # Query the todos table (after migrations)"
echo "  \\q               # Quit psql"
echo ""
echo "To test connection without interactive prompt:"
echo "  PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -d $DB_NAME -h localhost -c '\\dt'"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "NEXT STEPS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "1. Run database migrations:"
echo "   cd platform/api && cargo sqlx migrate run"
echo "   OR: make migrate"
echo ""
echo "2. Start the application:"
echo "   make dev"
echo ""
echo "3. Access the application:"
echo "   API:    http://localhost:8080"
echo "   Client: http://localhost:8081"
echo ""
echo "4. Test the API:"
echo "   curl http://localhost:8080/api/todos"
echo ""
echo "📚 Documentation: docs/README.md"
echo ""

