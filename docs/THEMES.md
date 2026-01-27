# Theme Management Guide

Complete guide to managing themes in the Atom Platform.

## Overview

The Atom Platform uses a **dual-source theme system**:

1. **JSON Files** (`themes/` directory) - Version controlled, easy to edit
2. **Database** (`themes` and `theme_settings` tables) - Runtime configuration

This approach gives you:
- ✅ Version control for themes (git)
- ✅ Easy bulk editing (JSON files)
- ✅ Live editing in UI (`/settings/theme`)
- ✅ Persistent storage (database)
- ✅ Team collaboration (share JSON files)

## Quick Start

### Option 1: Edit JSON Files

```bash
# 1. Edit theme files
vim themes/light.json

# 2. Compile to database
make compile-themes

# 3. Restart application
make stop && make dev
```

### Option 2: Edit in UI

```bash
# 1. Start application
make dev

# 2. Visit http://localhost:8081/settings/theme

# 3. Make changes in the theme editor

# 4. Export back to JSON (optional)
make export-themes
```

## Theme File Structure

```json
{
  "name": "Light",
  "is_active": true,
  "settings": {
    "colors": {
      "color_primary": {
        "value": "#3b82f6",
        "description": "Primary brand color"
      },
      "color_secondary": {
        "value": "#64748b",
        "description": "Secondary accent color"
      },
      "color_tertiary": {
        "value": "#10b981",
        "description": "Tertiary accent color"
      }
    },
    "spacing": {
      "spacing_md": {
        "value": "1rem",
        "description": "Medium spacing"
      }
    },
    "borders": {
      "radius_lg": {
        "value": "0.75rem",
        "description": "Large border radius"
      }
    },
    "typography": {
      "font_size_base": {
        "value": "1rem",
        "description": "Base font size"
      }
    }
  }
}
```

## Commands

### Compile Themes

Syncs JSON files to database:

```bash
make compile-themes
```

**What it does:**
- Reads all `*.json` files in `themes/` directory
- Creates themes if they don't exist
- Updates existing themes with new values
- Imports all settings per theme
- Preserves theme activation state

**Output:**
```
🎨 Atom Platform Theme Compiler
═══════════════════════════════

📂 Found 2 theme file(s)

Processing: dark.json
  Theme: Dark
  Active: False
  ✓ Updated theme: Dark
  ✓ Imported 16 settings
  ✓ Success

✅ Theme compilation complete!
```

### Export Themes

Exports database themes back to JSON:

```bash
make export-themes
```

**What it does:**
- Reads all themes from database
- Exports each theme with settings to JSON
- Organizes settings by category
- Overwrites existing JSON files

**Use this after:**
- Making changes in the UI
- Wanting to version control UI edits
- Sharing themes with team

## Creating Custom Themes

### Method 1: Copy Existing Theme

```bash
# Copy light theme as template
cp themes/light.json themes/custom-ocean.json

# Edit the file
{
  "name": "Ocean",
  "is_active": false,
  "settings": {
    "colors": {
      "color_primary": {
        "value": "#0891b2",  // Cyan
        "description": "Ocean blue primary"
      }
    }
  }
}

# Compile to database
make compile-themes
```

### Method 2: Create in UI

```bash
# 1. Go to /settings/theme
# 2. Click "+" button
# 3. Edit theme name and settings
# 4. Click "Activate Theme"
# 5. Export to JSON:
make export-themes
```

## Workflow Examples

### Scenario 1: Developer Editing Themes

```bash
# Edit theme JSON
vim themes/dark.json

# Compile changes
make compile-themes

# Rebuild API (if schema changed)
cd platform/api && cargo clean && cargo build

# Restart
make stop && make dev

# Verify at http://localhost:8081/settings/theme
```

### Scenario 2: Designer Using UI

```bash
# 1. Designer edits themes at /settings/theme
# 2. Developer exports changes:
make export-themes

# 3. Review changes:
git diff themes/

# 4. Commit if good:
git add themes/
git commit -m "Update dark theme colors"
```

### Scenario 3: Team Collaboration

```bash
# Developer A creates new theme
vim themes/custom-brand.json
git add themes/custom-brand.json
git commit -m "Add corporate brand theme"
git push

# Developer B pulls changes
git pull
make compile-themes  # Sync to their database
```

## Setting Categories

### Colors
- `color_primary` - Main brand color (buttons, links, accents)
- `color_secondary` - Secondary accent color
- `color_tertiary` - Success/highlight color

### Spacing
- `spacing_xs` - 0.25rem (4px)
- `spacing_sm` - 0.5rem (8px)
- `spacing_md` - 1rem (16px)
- `spacing_lg` - 1.5rem (24px)
- `spacing_xl` - 2rem (32px)

### Borders
- `radius_sm` - Small corner radius
- `radius_md` - Medium corner radius
- `radius_lg` - Large corner radius
- `radius_xl` - Extra large corner radius

### Typography
- `font_size_base` - Base font size (16px typically)
- `font_size_sm` - Small text
- `font_size_lg` - Large text
- `font_size_xl` - Extra large text

## Best Practices

### Version Control

**DO commit to git:**
- `themes/*.json` - Theme definitions
- `themes/README.md` - Documentation

**DON'T commit:**
- Individual edits without team review
- Auto-exported files without verification

### Naming Conventions

**Good:**
- `light.json` - Default light theme
- `dark.json` - Default dark theme
- `custom-corporate.json` - Descriptive name
- `custom-high-contrast.json` - Purpose-driven name

**Bad:**
- `theme1.json` - Not descriptive
- `new.json` - Ambiguous
- `test.json` - Temporary name

### Testing

After compiling themes:
1. Check `/settings/theme` - Verify tabs appear
2. Switch between themes - Test activation
3. Check colors - Verify buttons, badges use theme colors
4. Test dark mode - Ensure dark theme works
5. Visit `/showcase` - See components with theme colors

## Troubleshooting

### "Theme not appearing in UI"

```bash
# Verify JSON is valid
cat themes/your-theme.json | python3 -m json.tool

# Recompile
make compile-themes

# Check database
PGPASSWORD=atom psql -U atom -d atom -h localhost \
  -c "SELECT name, is_active FROM themes;"

# Rebuild and restart
cd platform/api && cargo clean && cargo build
make stop && make dev
```

### "Settings not updating"

```bash
# Export current state
make export-themes

# Review what's in database
PGPASSWORD=atom psql -U atom -d atom -h localhost \
  -c "SELECT theme.name, COUNT(settings.*) 
      FROM themes theme 
      LEFT JOIN theme_settings settings ON theme.id = settings.theme_id 
      GROUP BY theme.name;"

# Recompile from source
make compile-themes
```

### "Git conflicts in theme files"

```bash
# Keep both versions
git checkout --ours themes/light.json    # Keep your version
git checkout --theirs themes/light.json  # Keep their version

# Or merge manually
vim themes/light.json

# Then recompile
make compile-themes
```

## Integration with CI/CD

Add to your deployment pipeline:

```bash
# In CI/CD script
make setup-db
make migrate
make compile-themes  # Ensure themes are loaded
make build-api
make build-client
```

---

**Pro Tip**: Use `make export-themes` after major UI changes to keep JSON files in sync!
