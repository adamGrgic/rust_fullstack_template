# Atom Platform

A personal data management platform built with Rust, featuring a modern component library, dark/light themes, and admin interface.

## Features

- 🎯 **Todo Management** - Card and table views
- 🎨 **Theme System** - JSON-based themes with Light/Dark modes + custom themes
- 🧩 **Component Library** - 20+ Bootstrap-like UI components
- 🌓 **Dark Mode** - Full dark mode support with instant switching
- 📊 **Admin Panel** - Professional data management with DataTable
- ⚡ **Real-time Updates** - Live theme customization
- 🔄 **Version Controlled** - Themes managed via JSON files

## Quick Start

```bash
# 1. Setup tools and dependencies
./setup.sh

# 2. Setup database
./setup_db.sh

# 3. Run migrations and compile themes
make migrate
make compile-themes

# 4. Start the application
make dev
```

Visit: http://localhost:8081

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with SQLx
- **Frontend**: Leptos (WASM) with Tailwind CSS v4
- **Build**: Cargo workspace with Trunk

## Project Structure

```
atomplatform/
├── core/              # Shared types and models
├── platform/api/      # REST API server
├── client/            # Leptos frontend (WASM)
├── cli/               # CLI tool
├── themes/            # Theme JSON definitions
├── docs/              # Complete documentation
└── Makefile           # Development commands
```

## Documentation

📚 **[Complete Documentation →](./docs/README.md)**

- **Setup Guide**: `docs/SETUP.md`
- **Component Library**: `docs/COMPONENT_LIBRARY.md`
- **Theme Management**: `docs/THEMES.md`
- **Architecture**: `docs/claude.md`

## Development Commands

```bash
make dev             # Start everything
make stop            # Stop all services
make logs            # View logs
make compile-themes  # Sync JSON themes to DB
make export-themes   # Export DB themes to JSON
make migrate         # Run database migrations
```

See `make help` for all commands.

## Component Library

20+ production-ready components:

**Core**: Button, Badge, Card, Tabs  
**Forms**: Input, Textarea, FormGroup  
**Feedback**: Alert, Modal, Spinner, Progress  
**Data**: DataTable, Table, Pagination  
**Navigation**: SideNav, Dropdown, Breadcrumbs  

Preview all components: http://localhost:8081/showcase

## Theme Management

Themes are managed via JSON files:

```bash
# Edit theme
vim themes/light.json

# Compile to database
make compile-themes

# Or use UI at /settings/theme
```

## Database Credentials (Development)

- **Database**: `atom`
- **User**: `atom`
- **Password**: `atom`

⚠️ Change for production!

## URLs

- **Frontend**: http://localhost:8081
- **API**: http://localhost:8080
- **API Health**: http://localhost:8080/health

## License

MIT OR Apache-2.0

---

**[📖 Read the Full Documentation](./docs/README.md)** for detailed setup, component usage, and theme management guides.
