# Atom Platform Documentation

Welcome to the Atom Platform documentation. This directory contains comprehensive guides for setup, development, and component usage.

## 📚 Documentation Index

### Getting Started

- **[SETUP.md](./SETUP.md)** - Complete setup guide for fresh installations
  - Prerequisites and dependencies
  - Database setup
  - Running the application
  - Troubleshooting

### Development Guides

- **[claude.md](./claude.md)** - AI Agent reference and codebase overview
  - Project structure
  - Architecture decisions
  - Common patterns
  - Quick reference commands

- **[THEMES.md](./THEMES.md)** - Theme management system guide
  - JSON-based theme editing
  - Database synchronization
  - Workflow examples
  - Version control best practices

### Component Documentation

- **[COMPONENT_LIBRARY.md](./COMPONENT_LIBRARY.md)** - Complete component reference
  - 20+ production-ready components
  - Usage examples with code
  - Props documentation
  - Best practices

- **[COMPONENTS.md](./COMPONENTS.md)** - Component system overview
  - Theme system architecture
  - Database schema
  - Color system
  - API endpoints

## 🚀 Quick Links

### Setup & Installation
Start here if you're setting up the project for the first time:
1. [Prerequisites](./SETUP.md#prerequisites)
2. [Quick Setup](./SETUP.md#quick-setup)
3. [Running the Application](./SETUP.md#running-the-application)

### Development
For daily development tasks:
- [Makefile Commands](./SETUP.md#useful-commands)
- [Development Workflow](./claude.md#development-workflow)
- [Troubleshooting](./SETUP.md#troubleshooting)

### Component Usage
Building UI features:
- [Component Showcase](./COMPONENT_LIBRARY.md) (visit `/showcase` in the app)
- [Button Examples](./COMPONENT_LIBRARY.md#buttons)
- [Form Components](./COMPONENT_LIBRARY.md#form-components)
- [DataTable Guide](./COMPONENT_LIBRARY.md#datatable)

### Theming
Customizing the look and feel:
- [Theme Management](./THEMES.md)
- [Creating Custom Themes](./THEMES.md#creating-custom-themes)
- [JSON Format](./THEMES.md#theme-file-structure)

## 📖 Document Purposes

| Document | Purpose | Audience |
|----------|---------|----------|
| **SETUP.md** | Installation and configuration | New developers, DevOps |
| **claude.md** | Technical reference and architecture | AI agents, developers |
| **THEMES.md** | Theme system and workflow | Designers, developers |
| **COMPONENT_LIBRARY.md** | Component API reference | Frontend developers |
| **COMPONENTS.md** | System design and database schema | Architects, backend devs |

## 🔧 Common Tasks

### First Time Setup
```bash
./setup.sh
./setup_db.sh
make migrate
make compile-themes
make dev
```

### Daily Development
```bash
make dev          # Start everything
make logs         # View logs
make stop         # Stop everything
```

### Theme Development
```bash
vim themes/light.json
make compile-themes
make stop && make dev
```

### Component Development
```bash
# Edit component
vim client/src/components/ui/button.rs

# Rebuild
cd client && ./tailwindcss -i ./input.css -o ./style.css --minify

# Test at /showcase
```

## 🌐 Application URLs

When running locally:
- **Client**: http://localhost:8081
- **API**: http://localhost:8080
- **Health Check**: http://localhost:8080/health
- **Component Showcase**: http://localhost:8081/showcase
- **Theme Manager**: http://localhost:8081/settings/theme
- **Admin Todos**: http://localhost:8081/admin/todos

## 📝 Additional Resources

### External Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Guide](https://leptos.dev/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Guide](https://docs.rs/sqlx/)
- [Tailwind CSS v4](https://tailwindcss.com/)

### Project Files
- `Makefile` - All available commands
- `setup.sh` - Automated setup script
- `dev.sh` - Development server with monitoring
- `compile_themes.sh` - Theme compilation
- `export_themes.sh` - Theme export

## 🤝 Contributing

When contributing to the project:

1. **Read** - Start with [SETUP.md](./SETUP.md) for environment setup
2. **Understand** - Review [claude.md](./claude.md) for architecture
3. **Code** - Use components from [COMPONENT_LIBRARY.md](./COMPONENT_LIBRARY.md)
4. **Style** - Follow patterns in existing code
5. **Test** - Run `make test` and `make lint`
6. **Document** - Update relevant docs

## 📮 Getting Help

If you're stuck:
1. Check the relevant documentation file above
2. Search the troubleshooting sections
3. Review `make help` for available commands
4. Check logs: `make logs`
5. Open an issue with:
   - What you're trying to do
   - What's happening instead
   - Relevant log output
   - Your environment details

---

**Last Updated:** 2026-02-22  
**Version:** 0.1.0  
**Status:** Active Development
