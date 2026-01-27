# Themes Directory

This directory contains theme definition files in JSON format. Each theme file defines colors, spacing, borders, and typography settings.

## Structure

```
themes/
├── light.json      # Default light theme
├── dark.json       # Default dark theme
└── custom-*.json   # Your custom themes
```

## Theme File Format

```json
{
  "name": "Theme Name",
  "is_active": false,
  "settings": {
    "colors": {
      "color_primary": {
        "value": "#3b82f6",
        "description": "Primary brand color"
      }
    },
    "spacing": { ... },
    "borders": { ... },
    "typography": { ... }
  }
}
```

## Workflow

### 1. Edit Theme Files

Modify JSON files in this directory:

```bash
# Edit light theme
vim themes/light.json

# Create custom theme
cp themes/light.json themes/custom-corporate.json
# Edit the new file...
```

### 2. Compile to Database

Run the compile command to sync JSON files to the database:

```bash
make compile-themes
```

This will:
- Read all `*.json` files in the themes directory
- Create or update themes in the database
- Update all settings for each theme
- Preserve theme activation state

### 3. Verify in UI

Go to `/settings/theme` to see your themes as tabs and make live adjustments.

## Exporting Themes

To export current database themes back to JSON:

```bash
make export-themes
```

This will overwrite JSON files with current database values (useful after making changes in the UI).

## Version Control

The `themes/` directory should be committed to git, allowing you to:
- Track theme changes over time
- Share themes across team
- Roll back changes
- Review theme modifications in PRs

## Best Practices

1. **Don't edit generated files**: Files updated by `export-themes` should be reviewed before committing
2. **Use semantic names**: `themes/custom-dark-blue.json` not `themes/theme1.json`
3. **Document changes**: Add git commit messages when changing themes
4. **Test after compile**: Always check the UI after running `compile-themes`

## Tips

### Creating New Themes

1. Copy existing theme:
   ```bash
   cp themes/light.json themes/custom-ocean.json
   ```

2. Edit colors:
   ```json
   "color_primary": { "value": "#0891b2" }
   ```

3. Compile:
   ```bash
   make compile-themes
   ```

### Batch Updates

You can update multiple themes at once by editing JSON files then running `compile-themes` once.

### Reverting Changes

If you make a mistake:
```bash
git checkout themes/  # Revert to last commit
make compile-themes   # Apply reverted themes
```
