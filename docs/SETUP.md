# Atom Platform - Setup Guide

This guide will help you set up the Atom Platform development environment from a fresh clone.

## Quick Start (TL;DR)

For a quick setup from a fresh clone:

```bash
# 1. Setup tools and dependencies
./setup.sh

# 2. Setup PostgreSQL database
./setup_db.sh

# 3. Run database migrations
make migrate

# 4. Start development environment
make dev

# Open browser to http://localhost:8081
```

## Prerequisites

Before you begin, ensure you have the following installed:

### Required
- **Rust** (latest stable): Install from [https://rustup.rs/](https://rustup.rs/)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **PostgreSQL** (version 14+): 
  - **Linux/WSL**: `sudo apt install postgresql postgresql-contrib`
  - **macOS**: `brew install postgresql@16`
  - **Windows**: Download from [https://www.postgresql.org/download/windows/](https://www.postgresql.org/download/windows/)

### Rust Tools
- **wasm32-unknown-unknown target**: Required for building the WebAssembly client
- **Trunk**: Build tool for Rust/WASM web applications
- **Tailwind CSS CLI**: For styling (automatically downloaded by setup.sh)

## Quick Setup

We provide automated setup scripts:

**1. Install dependencies and tools:**
```bash
./setup.sh
```

This script will:
- Verify Rust is installed
- Check for PostgreSQL client
- Install Trunk (if not present)
- Install the wasm32-unknown-unknown target (if not present)
- Download Tailwind CSS CLI (if not present)
- Create environment configuration files

**2. Setup database (after PostgreSQL is installed):**
```bash
make setup-db
```
or
```bash
./setup_db.sh
```
**Note:** Run as your regular user (not with sudo). The script will prompt for sudo when needed.

This script will:
- Create the `atom` database (if it doesn't exist)
- Create the `atom` user (if it doesn't exist)
- Grant necessary permissions
- Update `platform/api/.env` with the correct DATABASE_URL

**3. Run migrations and build:**
```bash
make migrate
cd platform/api && cargo build
```
**Note:** Building after migrations ensures SQLx has the latest schema compiled.

**4. Start the application:**
```bash
make dev
```

**5. View logs (in another terminal):**
```bash
make logs          # View all logs
make api-logs      # View API logs only
make client-logs   # View client logs only
```

## Manual Setup

If you prefer to set up manually or need to troubleshoot:

### 1. Clone the Repository

```bash
git clone <repository-url>
cd atomplatform
```

### 2. Install Rust Tools

```bash
# Add WASM target for client builds
rustup target add wasm32-unknown-unknown

# Install Trunk for serving the WASM client
cargo install trunk
```

### 3. Install PostgreSQL

#### Linux/WSL

```bash
# Install PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Start PostgreSQL service
sudo service postgresql start

# Verify it's running
sudo service postgresql status
```

#### macOS

```bash
# Install PostgreSQL
brew install postgresql@16

# Start PostgreSQL service
brew services start postgresql@16

# Verify it's running
brew services list | grep postgresql
```

#### Windows

1. Download and install PostgreSQL from [https://www.postgresql.org/download/windows/](https://www.postgresql.org/download/windows/)
2. During installation, set a password for the postgres user (remember this!)
3. Ensure the service is running in the Services app (`services.msc`)

### 4. Setup Database and User

We provide an automated script that will:
- Create the `atom` database (if it doesn't exist)
- Create the `atom` user (if it doesn't exist)
- Grant necessary permissions
- Update your `.env` file with the correct DATABASE_URL

```bash
./setup_db.sh
```

#### Manual Database Setup (Alternative)

If you prefer to set up manually:

**Linux/WSL:**
```bash
sudo -u postgres psql -c "CREATE DATABASE atom;"
sudo -u postgres psql -c "CREATE USER atom WITH PASSWORD 'atom';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE atom TO atom;"
sudo -u postgres psql -d atom -c "GRANT ALL ON SCHEMA public TO atom;"
```

**macOS:**
```bash
createdb atom
psql postgres -c "CREATE USER atom WITH PASSWORD 'atom';"
psql postgres -c "GRANT ALL PRIVILEGES ON DATABASE atom TO atom;"
psql atom -c "GRANT ALL ON SCHEMA public TO atom;"
```

**Windows (SQL Shell):**
```sql
CREATE DATABASE atom;
CREATE USER atom WITH PASSWORD 'atom';
GRANT ALL PRIVILEGES ON DATABASE atom TO atom;
\c atom
GRANT ALL ON SCHEMA public TO atom;
\q
```

### 5. Run Database Migrations

```bash
cd platform/api
cargo sqlx migrate run
```

This will create all necessary database tables and schemas.

## Running the Application

### Development Mode (Recommended)

The easiest way to start everything with monitoring:

```bash
make dev
```

This will:
1. Start the API server on `http://localhost:8080`
2. Start the client dev server on `http://localhost:8081`
3. Display a live status dashboard
4. Monitor both services and restart if needed
5. Automatically manage logs in `.logs/` directory

**Status Dashboard:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
         🚀 Atom Platform - Development Environment
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Services Status:
  Platform API        ● running     http://localhost:8080
  Client UI           ● running     http://localhost:8081
```

**To stop all services:**
```bash
make stop
```
Or press `Ctrl+C` in the terminal running `make dev`

### Running Components Separately

If you prefer to run services individually:

**Terminal 1 - API Server:**
```bash
cd platform/api
cargo run
```

**Terminal 2 - Client:**
```bash
cd client
trunk serve
```

**Terminal 3 - CLI (optional):**
```bash
cd cli
cargo run -- --help
```

## Project Structure

```
atomplatform/
├── core/              # Core domain models and business logic
├── platform/
│   └── api/          # REST API server (Axum)
├── client/           # Frontend client (Leptos/WASM)
├── cli/              # Command-line interface tool
├── Cargo.toml        # Workspace configuration
├── Makefile          # Development commands
├── dev.sh            # Development server with monitoring
├── logs.sh           # Log viewer with color-coded errors
├── setup.sh          # Initial setup script
├── setup_db.sh       # Database setup script
├── clean_db.sh       # Database cleanup script
├── SETUP.md          # This file
└── .logs/            # Development logs (gitignored)
```

## Styling & Themes

### Tailwind CSS

The client uses **Tailwind CSS v4.2.0** for styling:

- **Configuration**: `client/tailwind.config.js`
- **Source CSS**: `client/input.css` (with `@import "tailwindcss"`)
- **Output CSS**: `client/style.css` (auto-generated, gitignored)
- **CLI Binary**: `client/tailwindcss` (auto-downloaded by `setup.sh`, gitignored)

Tailwind runs automatically during the build process via a Trunk pre-build hook.

### Theme Management

Themes are managed via JSON files in the `themes/` directory:

```bash
themes/
├── light.json      # Default light theme
├── dark.json       # Default dark theme
├── custom-*.json   # Your custom themes
└── README.md       # Theme documentation
```

**Compile themes to database:**
```bash
make compile-themes
```

**Export database themes to JSON:**
```bash
make export-themes
```

**Workflow:**
1. Edit JSON files in `themes/` directory
2. Run `make compile-themes` to sync to database
3. Restart app to see changes: `make stop && make dev`
4. Or use UI at `/settings/theme` for live editing
5. Run `make export-themes` to save UI changes back to JSON

See `themes/README.md` for detailed documentation.

## Building for Production

### Build API
```bash
cd platform/api
cargo build --release
```

The binary will be at `platform/api/target/release/platform-api`

### Build Client
```bash
cd client
trunk build --release
```

The static files will be in `client/dist/`

### Build CLI
```bash
cd cli
cargo build --release
```

The binary will be at `cli/target/release/cli`

## Useful Commands

The project includes a Makefile with common commands:

```bash
make help            # Show all available commands
make setup           # Run initial setup (tools and dependencies)
make setup-db        # Setup PostgreSQL database and user
make clean-db        # Remove PostgreSQL database and user (DESTRUCTIVE)
make migrate         # Run database migrations
make dev             # Start full dev environment (API + Client)
make stop            # Stop all running services
make api             # Run API server
make api-logs        # Tail API logs with color-coding
make client          # Run client dev server
make client-logs     # Tail client logs
make logs            # Tail all logs
make build-api       # Build API for production
make build-client    # Build client for production
make compile-themes  # Compile JSON themes to database
make export-themes   # Export database themes to JSON
make test            # Run all tests
make fmt             # Format code
make lint            # Run clippy linter
make clean           # Clean build artifacts
```

## Cargo Workspace

This project uses Cargo workspace for dependency management. All crates share:
- Common dependencies defined in the root `Cargo.toml`
- Unified versioning (0.1.0)
- Shared Cargo.lock for consistent builds

To work with the workspace:

```bash
# Check all crates
cargo check --workspace

# Test all crates
cargo test --workspace

# Build all crates
cargo build --workspace

# Format all code
cargo fmt --all

# Run clippy on all crates
cargo clippy --all
```

## Troubleshooting

### Database Connection Issues

If you see "connection refused" errors:
1. Ensure PostgreSQL is running:
   - **Linux/WSL**: `sudo service postgresql status`
   - **macOS**: `brew services list | grep postgresql`
   - **Windows**: Check Services app (`services.msc`) for "postgresql" service
2. Check the DATABASE_URL in `platform/api/.env` matches your PostgreSQL credentials
3. Verify you can connect manually: 
   - `psql -U atom -d atom -h localhost` (password: atom)
   - Or as postgres admin: `sudo -u postgres psql -d atom` (Linux/WSL)
4. Check PostgreSQL is listening on port 5432: `sudo lsof -i :5432` (Linux/Mac) or `netstat -an | findstr 5432` (Windows)

### Authentication Issues

If you get "password authentication failed" errors:
1. Verify the DATABASE_URL in `platform/api/.env` has the correct credentials
2. Try running `./setup_db.sh` again to ensure user and permissions are set correctly
3. On Linux/WSL, you may need to configure `pg_hba.conf`:
   ```bash
   sudo nano /etc/postgresql/*/main/pg_hba.conf
   # Add this line before other rules:
   # local   all   atom   md5
   # Then restart: sudo service postgresql restart
   ```
4. Test connection manually: `psql -U atom -d atom -h localhost` (password: atom)

### Resetting the Database

If you need to completely remove and recreate the database:

```bash
# Remove database and user
./clean_db.sh
# or
make clean-db

# Then recreate everything
./setup_db.sh
make migrate
```

**Warning:** This will delete all data in the database!

### SQLX Compile-Time Verification Errors

If you see errors about `DATABASE_URL` during compilation:
1. Ensure the database is running
2. Ensure the `.env` file exists with correct DATABASE_URL (run `./setup_db.sh` if needed)
3. Run migrations: `cd platform/api && cargo sqlx migrate run`
4. Do a clean build: `cargo clean && cargo build`
5. Alternatively, set the DATABASE_URL environment variable:
   ```bash
   export DATABASE_URL=postgres://atom:atom@localhost:5432/atom
   ```

Alternatively, use offline mode:
```bash
cd platform/api
cargo sqlx prepare
```

### Migration Issues ("VersionMissing" or "cached plan" errors)

If the API fails to start after adding/running migrations:
1. Do a clean rebuild: `cd platform/api && cargo clean && cargo build`
2. Restart all services: `make stop && make dev`
3. Verify migrations: `PGPASSWORD=atom psql -U atom -d atom -h localhost -c "SELECT * FROM _sqlx_migrations;"`

### Client Build Issues

If the client fails to build:
1. Verify wasm32 target is installed: `rustup target list | grep wasm32`
2. Ensure Trunk is installed: `trunk --version`
3. Ensure Tailwind CSS CLI is present: `ls client/tailwindcss` (run `./setup.sh` if missing)
4. Try cleaning and rebuilding: `cd client && trunk clean && trunk build`
5. Manually rebuild Tailwind: `cd client && ./tailwindcss -i ./input.css -o ./style.css --minify`

### Port Already in Use

If ports 8080 or 8081 are already in use:
- Run `make stop` to kill all services on those ports
- Or manually: `lsof -ti:8080,8081 | xargs kill`
- Alternatively, change the API port in `platform/api/src/main.rs`
- Or change the client port by running: `trunk serve --port <PORT>`

## Features

The Atom Platform includes:

- **Todo Management**: Create, read, update, delete todos
- **Dual Views**: Card layout (home) and table layout (admin)
- **Component Library**: Reusable UI components (Button, Card, Table, SideNav)
- **Theme System**: Customizable colors (primary, secondary, tertiary)
- **Live Customization**: Adjust colors, spacing, borders in real-time via Settings
- **Admin Panel**: Professional layout with sidebar navigation

### Pages

- **Home** (`/`) - Card-based todo view
- **Admin Todos** (`/admin/todos`) - Table view with admin actions  
- **Settings > Components** (`/settings/components`) - Live theme editor

## Additional Resources

- **Project Documentation**: See `COMPONENTS.md` for component system details
- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Axum Documentation**: [https://docs.rs/axum/](https://docs.rs/axum/)
- **Leptos Documentation**: [https://leptos.dev/](https://leptos.dev/)
- **SQLx Documentation**: [https://docs.rs/sqlx/](https://docs.rs/sqlx/)
- **Trunk Documentation**: [https://trunkrs.dev/](https://trunkrs.dev/)
- **Tailwind CSS v4**: [https://tailwindcss.com/](https://tailwindcss.com/)

## Development Logs

When running `make dev`, logs are automatically saved to:
- **API logs**: `.logs/api.log`
- **Client logs**: `.logs/client.log`

View live logs with color-coded errors:
```bash
make logs          # View both logs
make api-logs      # View API logs only  
make client-logs   # View client logs only
```

Or manually:
```bash
tail -f .logs/api.log
tail -f .logs/client.log
./logs.sh both     # Colored output
```

Recent errors are also displayed in the status dashboard that appears when running `make dev`.

## Getting Help

If you encounter issues not covered here:
1. Check the logs in `.logs/` directory
2. Review existing GitHub issues
3. Check `claude.md` for AI agent reference documentation
4. Open a new issue with details about your environment and the error

Happy coding!
