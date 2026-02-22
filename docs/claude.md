# Atom Platform - AI Agent Reference

This document provides a comprehensive overview of the Atom Platform codebase for AI agents working on this project.

## Project Overview

**Type:** Personal data management platform with admin interface and component system  
**Language:** Rust  
**Architecture:** Monorepo workspace with separate API, client, and shared core library  
**Database:** PostgreSQL with SQLx  
**Frontend:** Leptos (WASM-based reactive UI) with Tailwind CSS v4  
**Features:**
- Todo management (card and table views)
- Reusable UI component library (Button, Card, DataTable, SideNav, ThemeToggle)
- Dark/Light mode with instant switching
- Live theme customization system
- Admin panel with sidenav navigation
- localStorage caching for instant theme loading (no flash)

## Workspace Structure

```
atomplatform/
├── Cargo.toml              # Workspace root with shared dependencies
├── Makefile                # Common development commands
├── dev.sh                  # Development server with monitoring
├── logs.sh                 # Log viewer with color-coding
├── setup.sh                # Initial project setup
├── setup_db.sh             # Database setup automation
├── clean_db.sh             # Database cleanup
├── compile_themes.sh       # Compile JSON themes to database
├── export_themes.sh        # Export database themes to JSON
├── SETUP.md                # Setup documentation
├── COMPONENT_LIBRARY.md    # Complete component reference
├── themes/                 # Theme JSON definitions (version controlled)
│   ├── light.json          # Light theme configuration
│   ├── dark.json           # Dark theme configuration
│   └── README.md           # Theme management guide
├── core/                   # Shared types library (workspace member)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Re-exports
│       └── todo.rs         # Todo types: Todo, TodoCreate, TodoUpdate, TodoStatus
├── platform/
│   └── api/                # REST API server (workspace member)
│       ├── Cargo.toml
│       ├── env.example     # Environment template
│       ├── migrations/     # SQLx migrations (run on startup)
│       │   └── *.sql
│       └── src/
│           ├── main.rs     # Server entry, runs migrations
│           ├── config.rs   # Env config loading
│           ├── db.rs       # Database connection pool
│           ├── error.rs    # AppError type with IntoResponse
│           ├── routes.rs   # Route definitions
│           └── handlers/
│               └── todos.rs # CRUD handlers
├── client/                 # Leptos web client (workspace member)
│   ├── Cargo.toml
│   ├── Trunk.toml          # Build configuration with Tailwind hook
│   ├── index.html          # HTML entry point
│   ├── input.css           # Tailwind source (with @import)
│   ├── style.css           # Generated Tailwind CSS (gitignored)
│   ├── tailwind.config.js  # Tailwind configuration (v4)
│   ├── tailwindcss         # Tailwind CLI binary (gitignored)
│   └── src/
│       ├── lib.rs          # WASM entry with App, SideNav, routing
│       ├── api.rs          # HTTP client for API (todos + settings)
│       ├── components/
│       │   ├── mod.rs
│       │   ├── todo_list.rs   # Card-based todo list
│       │   ├── todo_item.rs   # Individual todo card
│       │   ├── todo_form.rs   # Create todo form
│       │   └── ui/            # Reusable component library
│       │       ├── mod.rs
│       │       ├── button.rs     # Button with variants
│       │       ├── card.rs       # Card container
│       │       ├── table.rs      # Table components
│       │       └── sidenav.rs    # Navigation sidebar
│       └── pages/
│           ├── mod.rs
│           ├── home.rs                 # Card view (/)
│           ├── admin_todos.rs          # Table view (/admin/todos)
│           └── settings_components.rs  # Theme editor (/settings/components)
├── cli/                    # CLI tool (workspace member)
│   └── src/
│       ├── main.rs
│       ├── etsy.rs         # Etsy API integration
│       └── printful.rs     # Printful API integration
└── .logs/                  # Development logs (gitignored)
    ├── api.log
    └── client.log
```

## Core Concepts

### 1. Workspace Dependencies

All dependencies are defined in the root `Cargo.toml` under `[workspace.dependencies]`. Individual crates reference them with `.workspace = true`. This ensures version consistency.

### 2. Shared Types (platform-core)

**Location:** `core/src/todo.rs`

**Key Types:**
- `TodoStatus`: Enum (Pending, InProgress, Completed, Cancelled)
  - Has SQLx support via optional `sqlx` feature
  - Maps to `todo_status` PostgreSQL enum
- `Todo`: Complete todo with all fields (id, title, description, status, timestamps)
- `TodoCreate`: Request body for creating todos
- `TodoUpdate`: Request body for updating todos (all fields optional)

**Important:** 
- These types are used by both API (database mapping) and client (HTTP requests)
- API enables `sqlx` feature: `platform-core = { path = "../../core", features = ["sqlx"] }`
- Client uses core without sqlx to avoid pulling in database dependencies to WASM

### 3. API Server (platform-api)

**Framework:** Axum 0.7  
**Database:** SQLx with PostgreSQL  
**Port:** 8080 (configurable via PORT env var)

#### Endpoints

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/health` | `health_check` | Health check endpoint |
| GET | `/api/todos` | `todos::list_todos` | List all todos |
| GET | `/api/todos/:id` | `todos::get_todo` | Get single todo |
| POST | `/api/todos` | `todos::create_todo` | Create new todo |
| PUT | `/api/todos/:id` | `todos::update_todo` | Update existing todo |
| DELETE | `/api/todos/:id` | `todos::delete_todo` | Delete todo |
| GET | `/api/settings` | `component_settings::list_settings` | List all component settings |
| GET | `/api/settings/:id` | `component_settings::get_setting` | Get single setting |
| PUT | `/api/settings/:id` | `component_settings::update_setting` | Update setting value |
| GET | `/api/themes` | `themes::list_themes` | List all themes |
| GET | `/api/themes/:id` | `themes::get_theme_with_settings` | Get theme with settings |
| POST | `/api/themes` | `themes::create_theme` | Create new theme |
| PUT | `/api/themes/:id` | `themes::update_theme` | Update theme (name/activate) |
| DELETE | `/api/themes/:id` | `themes::delete_theme` | Delete theme (if not active) |
| PUT | `/api/themes/:theme_id/settings/:setting_id` | `themes::update_theme_setting` | Update setting for specific theme |

#### Database

**Connection:** Managed by `Database` struct wrapping `PgPool`  
**Migrations:** Located in `platform/api/migrations/`, run automatically on startup via `db.run_migrations()`  
**Schema:** Single `todos` table with UUID primary key, text fields, status enum, and timestamps

**Query Pattern:**
```rust
sqlx::query_as!(
    Todo,
    r#"SELECT id, title, description, 
       status as "status: TodoStatus", 
       created_at, updated_at 
       FROM todos WHERE id = $1"#,
    id
)
```

The `as "status: TodoStatus"` syntax tells SQLx to map the database text to the Rust enum.

#### Error Handling

Custom `AppError` enum in `error.rs`:
- `Database(sqlx::Error)`: Database errors → 500
- `NotFound`: Resource not found → 404
- `BadRequest(String)`: Validation errors → 400

Implements `IntoResponse` for automatic HTTP response conversion.

#### Configuration

Loaded from environment variables via `Config::from_env()`:
- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 8080)
- `RUST_LOG`: Logging level

### 4. Client (client)

**Framework:** Leptos 0.6 (CSR mode)  
**Build Tool:** Trunk  
**Port:** 8081 (dev server)

**Entry Point:** `#[wasm_bindgen(start)]` function in `lib.rs` - automatically called when WASM loads

#### Component Hierarchy

```
App (lib.rs)
└── HomePage
    └── TodoList (todo_list.rs)
        ├── TodoForm (todo_form.rs)
        └── For loop → TodoItem (todo_item.rs)
```

**Note:** Client is library-only (no `main.rs`), uses `wasm_bindgen(start)` for initialization

#### State Management

**Signals:** Reactive primitives from Leptos
- `create_signal(T)`: Returns `(ReadSignal<T>, WriteSignal<T>)`
- Updates automatically trigger re-renders

**Effects:** Side effects with `create_effect()`
- Used in `TodoList` to fetch todos on mount

**Async:** `spawn_local()` for async operations in WASM

#### API Client

**Location:** `client/src/api.rs`  
**HTTP Library:** gloo-net  
**Base URL:** `http://localhost:8080/api` (hardcoded constant)

**Functions:**
- `fetch_todos() -> Result<Vec<Todo>, String>`
- `create_todo(TodoCreate) -> Result<Todo, String>`
- `update_todo(Uuid, TodoUpdate) -> Result<Todo, String>`
- `delete_todo(Uuid) -> Result<(), String>`

#### Component Communication

Parent-to-child: Props  
Child-to-parent: Callbacks (e.g., `on_created`, `on_updated`, `on_deleted`)

Example: `TodoList` passes callbacks to `TodoItem` to handle updates.

## Development Workflow

### Quick Start (Recommended)

```bash
# 1. Run initial setup (installs Rust tools)
./setup.sh

# 2. Setup database (creates DB and user)
./setup_db.sh
# OR: make setup-db

# 3. Run migrations
make migrate

# 4. Start development servers with monitoring
make dev
```

This starts both API and client with live status dashboard and automatic log management.

### Manual Setup

**Terminal 1 - API:**
```bash
cd platform/api
cargo run
```

**Terminal 2 - Client:**
```bash
cd client
trunk serve
```

**Access:**
- API: http://localhost:8080
- Client: http://localhost:8081

### Database Setup

**Automated (Recommended):**
```bash
./setup_db.sh
```

This creates:
- Database: `atom`
- User: `atom` with password `atom`
- Grants all necessary permissions
- Updates `.env` file

**Manual:**
```bash
# Install PostgreSQL (if needed)
sudo apt install postgresql postgresql-contrib  # Linux/WSL
brew install postgresql@16                      # macOS

# Create database and user
sudo -u postgres psql -c "CREATE DATABASE atom;"
sudo -u postgres psql -c "CREATE USER atom WITH PASSWORD 'atom';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE atom TO atom;"

# Configure API
cp platform/api/env.example platform/api/.env
# Edit .env with: DATABASE_URL=postgres://atom:atom@localhost:5432/atom

# Migrations run automatically on API startup
```

### Building

**API:**
```bash
cd platform/api
cargo build --release
```

**Client:**
```bash
cd client
trunk build --release
# Output in dist/
```

## Common Modification Patterns

### Adding a New Field to Todo

1. Update `platform/core/src/todo.rs`:
   - Add field to `Todo` struct
   - Add to `TodoCreate` or `TodoUpdate` as appropriate

2. Create migration in `platform/api/migrations/`:
   ```sql
   ALTER TABLE todos ADD COLUMN new_field TEXT;
   ```

3. Update queries in `platform/api/src/handlers/todos.rs`:
   - Add field to SELECT statements
   - Add to INSERT/UPDATE statements

4. Update client components in `client/src/components/`:
   - Display new field in `todo_item.rs`
   - Add input in `todo_form.rs` if needed

### Adding a New Endpoint

1. Add handler function in `platform/api/src/handlers/`
2. Add route in `platform/api/src/routes.rs`
3. Add client function in `client/src/api.rs`
4. Use in appropriate component

### Adding a New Component

1. Create file in `client/src/components/`
2. Add module declaration in `client/src/components/mod.rs`
3. Import and use in parent component

## Key Dependencies

### API
- `axum = "0.7"`: Web framework
- `sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }`: Database
- `tower-http`: CORS and middleware
- `tokio`: Async runtime

### Client
- `leptos = { version = "0.6", features = ["csr"] }`: UI framework
- `gloo-net`: HTTP client for WASM
- `wasm-bindgen`: WASM bindings
- **Tailwind CSS v4.2.0**: Utility-first CSS framework (standalone CLI)

### Shared
- `serde`: Serialization
- `uuid`: Unique identifiers
- `chrono`: Timestamps

## Testing

Currently no tests implemented. To add:

**API Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Use axum::test helpers
}
```

**Client Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Use leptos testing utilities
}
```

## Debugging

### API Debugging
- Check logs (configured via `RUST_LOG` env var)
- Use `tracing::debug!()`, `tracing::info!()`, etc.
- Test endpoints with curl or Postman

### Client Debugging
- Open browser console (F12)
- Logs appear via `console_log` crate
- Use `log::debug!()`, `log::info!()`, etc.

### Database Debugging
- Connect directly: `psql $DATABASE_URL`
- Check migrations: `SELECT * FROM _sqlx_migrations;`
- View data: `SELECT * FROM todos;`

## Known Limitations

1. **No Authentication**: All endpoints are public
2. **No Pagination**: All todos loaded at once
3. **No Real-time Updates**: Manual refresh needed
4. **Hardcoded API URL**: Client has hardcoded localhost URL
5. **No Error Recovery**: Failed requests don't retry
6. **No Optimistic Updates**: UI waits for server response

## Future Enhancements

Based on the project structure, likely future additions:

1. **CLI Connectors** (`cli/` directory exists but empty)
   - Third-party data source integrations
   - Import/export functionality

2. **Authentication**
   - User accounts
   - JWT tokens
   - Protected endpoints

3. **Additional Data Types**
   - Notes, tasks, events, etc.
   - Following the same pattern as todos

4. **Advanced Features**
   - Search and filtering
   - Tags and categories
   - File attachments
   - Sharing and collaboration

## Code Style

- Use `rustfmt` for formatting
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Prefer explicit error handling over unwrap/expect
- Use type aliases for common Result types
- Document public APIs with doc comments

## Environment Variables

**API (.env file in platform/api/):**
```bash
DATABASE_URL=postgres://atom:atom@localhost:5432/atom
PORT=8080
RUST_LOG=platform_api=debug,tower_http=debug
```

**Development Credentials (DO NOT use in production):**
- Database: `atom`
- Username: `atom`
- Password: `atom`

**Client:**
No environment variables currently used. API URL is hardcoded in `api.rs` as `http://localhost:8080/api`.

## Deployment Considerations

**API:**
- Needs PostgreSQL database
- Set environment variables
- Run migrations on first deploy
- Consider using a process manager (systemd, docker)

**Client:**
- Build with `trunk build --release`
- Serve `dist/` directory as static files
- Update API URL in `api.rs` before building
- Consider using CDN for assets

**Database:**
- Ensure PostgreSQL is accessible
- Use connection pooling (already configured in SQLx)
- Regular backups recommended
- Consider read replicas for scaling

## Troubleshooting Common Issues

### "Database not found" / "connection refused"
- Run database setup: `./setup_db.sh` or `make setup-db`
- Check PostgreSQL is running:
  - Linux/WSL: `sudo service postgresql status`
  - macOS: `brew services list | grep postgresql`
- Verify DATABASE_URL in `platform/api/.env`

### "Cannot find type TodoStatus in this scope"
- Ensure `platform-core` is in dependencies
- For API, check sqlx feature is enabled: `platform-core = { path = "../../core", features = ["sqlx"] }`
- Check import: `use platform_core::TodoStatus;`

### "Port 8080 or 8081 already in use"
- Run: `make stop`
- Or manually: `lsof -ti:8080,8081 | xargs kill`

### "CORS error in browser"
- API has CORS configured to allow all origins
- Check API is running: `curl http://localhost:8080/api/todos`
- Check client is accessing correct URL

### "SQLx compile error: TodoStatus doesn't implement Decode"
- Ensure core crate has sqlx feature enabled in API's Cargo.toml
- Check TodoStatus has `#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]`
- Verify DATABASE_URL is set for compile-time query verification
- Ensure database has `todo_status` enum type created via migration

### "Migrate(VersionMissing)" or "cached plan must not change result type"
- This happens after adding new migrations or changing column types
- Solution: Clean rebuild required
  ```bash
  cd platform/api
  cargo clean && cargo build
  make stop && make dev
  ```
- Always rebuild after running new migrations

### "Trunk build fails" / "NO_COLOR error"
- Trunk is called with `unset NO_COLOR` in Makefile
- Install wasm32 target: `rustup target add wasm32-unknown-unknown`
- Install trunk: `cargo install trunk`
- Clear cache: `trunk clean && cargo clean`

### "Client binary/library name collision"
- Client should only have `lib.rs`, not `main.rs`
- Uses `#[wasm_bindgen(start)]` for automatic initialization

## Makefile Commands

```bash
make help          # Show all available commands
make setup         # Run initial setup (tools and dependencies)
make setup-db      # Setup PostgreSQL database and user
make clean-db      # Remove PostgreSQL database and user (DESTRUCTIVE)
make migrate       # Run database migrations
make dev           # Start development environment with monitoring
make stop          # Stop all running services (ports 8080 and 8081)
make api           # Run API server only
make api-logs      # Tail API logs with color-coded errors
make client        # Run client dev server only
make client-logs   # Tail client logs
make logs          # Tail all logs (both API and client)
make build-api     # Build API for production
make build-client  # Build client for production
make test          # Run all tests
make fmt           # Format code
make lint          # Run clippy linter
make clean         # Clean build artifacts
```

## Quick Reference Commands

```bash
# Setup from scratch
./setup.sh && ./setup_db.sh && make migrate
cd platform/api && cargo build  # Build after migrations

# Start everything
make dev

# Stop everything
make stop  # OR press Ctrl+C in dev terminal

# View logs (with color-coded errors)
make logs          # Both API and client
make api-logs      # API only
make client-logs   # Client only
./logs.sh [api|client|both]  # Direct script

# Manual database operations
PGPASSWORD=atom psql -U atom -d atom -h localhost  # Connect to DB
# Or interactively: psql -U atom -d atom -h localhost (enter: atom)

# Build for production
make build-api
make build-client

# Format and check code
make fmt
make lint
cargo check --workspace

# Database operations
make setup-db      # Create DB and user
make clean-db      # Remove DB and user
make migrate       # Run migrations

# After adding migrations, always:
cd platform/api && cargo clean && cargo build
```

## Development Tools

### Status Monitoring

The `dev.sh` script provides live status monitoring:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
         🚀 Atom Platform - Development Environment
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Services Status:
  Platform API        ● running     http://localhost:8080
  Client UI           ● running     http://localhost:8081

Logs:
  → API:    tail -f .logs/api.log
  → Client: tail -f .logs/client.log
```

Features:
- Automatic port conflict detection
- Service health monitoring (updates every 5s)
- Automatic process cleanup on exit
- Separate log files for each service
- Color-coded status indicators

### Setup Scripts

**setup.sh** - Initial project setup
- Verifies Rust installation
- Checks for PostgreSQL
- Installs Trunk and wasm32 target
- Creates initial .env file

**setup_db.sh** - Database setup
- Creates `atom` database
- Creates `atom` user with password `atom`
- Grants all permissions
- Updates .env with DATABASE_URL
- Interactive prompts with clear status messages

**clean_db.sh** - Database cleanup
- Drops `atom` database
- Drops `atom` user
- Requires confirmation before proceeding
- Preserves .env file

---

**Last Updated:** 2026-02-22  
**Rust Version:** 1.75+ recommended  
**Database Version:** PostgreSQL 14+  
**Tailwind CSS:** v4.2.0 (standalone CLI)  
**Key Changes:**
- Core moved to root level (sibling of platform)
- Database changed from `atomplatform` to `atom` (user: atom, pass: atom)
- Added development monitoring with `dev.sh`
- Client now library-only (no main.rs)
- SQLx feature for core crate
- Tailwind CSS v4 for styling with utility classes
- Automated log management and error visibility

