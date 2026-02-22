#!/bin/bash

# Atom Platform Theme Exporter
# Exports database themes to JSON files

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

THEMES_DIR="themes"
DB_USER="atom"
DB_PASSWORD="atom"

echo -e "${CYAN}📤 Atom Platform Theme Exporter${NC}"
echo -e "${CYAN}═══════════════════════════════${NC}"
echo ""

# Create themes directory if it doesn't exist
mkdir -p "$THEMES_DIR"

# Export themes using Python
PGPASSWORD=$DB_PASSWORD python3 - <<'PYTHON_SCRIPT'
import json
import subprocess
import re

def sanitize_filename(name):
    """Convert theme name to safe filename"""
    return re.sub(r'[^a-z0-9-]', '-', name.lower()).strip('-')

# Get all themes
result = subprocess.run(
    ['psql', '-U', 'atom', '-d', 'atom', '-h', 'localhost', '-t', '-c', 
     'SELECT id, name, is_active FROM themes ORDER BY name'],
    capture_output=True,
    text=True,
    env={'PGPASSWORD': 'atom'}
)

if result.returncode != 0:
    print(f"Error fetching themes: {result.stderr}")
    exit(1)

themes = []
for line in result.stdout.strip().split('\n'):
    if not line.strip():
        continue
    parts = [p.strip() for p in line.split('|')]
    if len(parts) >= 3:
        themes.append({
            'id': parts[0],
            'name': parts[1],
            'is_active': parts[2] == 't'
        })

print(f"Found {len(themes)} theme(s)\n")

# Export each theme
for theme in themes:
    theme_id = theme['id']
    theme_name = theme['name']
    is_active = theme['is_active']
    
    print(f"Exporting: {theme_name}")
    
    # Get settings for this theme
    settings_result = subprocess.run(
        ['psql', '-U', 'atom', '-d', 'atom', '-h', 'localhost', '-t', '-c',
         f"SELECT key, value, category, description FROM theme_settings WHERE theme_id = '{theme_id}' ORDER BY category, key"],
        capture_output=True,
        text=True,
        env={'PGPASSWORD': 'atom'}
    )
    
    if settings_result.returncode != 0:
        print(f"  ✗ Error fetching settings: {settings_result.stderr}")
        continue
    
    # Organize settings by category
    theme_json = {
        'name': theme_name,
        'is_active': is_active,
        'settings': {}
    }
    
    for line in settings_result.stdout.strip().split('\n'):
        if not line.strip():
            continue
        parts = [p.strip() for p in line.split('|')]
        if len(parts) >= 4:
            key = parts[0]
            value = parts[1]
            category = parts[2]
            description = parts[3] if parts[3] else ''
            
            if category not in theme_json['settings']:
                theme_json['settings'][category] = {}
            
            theme_json['settings'][category][key] = {
                'value': value,
                'description': description
            }
    
    # Write to file
    filename = f"themes/{sanitize_filename(theme_name)}.json"
    with open(filename, 'w') as f:
        json.dump(theme_json, f, indent=2)
    
    setting_count = sum(len(settings) for settings in theme_json['settings'].values())
    print(f"  ✓ Exported {setting_count} settings to {filename}")

print("\n✅ Export complete!")

PYTHON_SCRIPT

echo ""
echo -e "${GREEN}═══════════════════════════════${NC}"
echo -e "${GREEN}✅ Theme export complete!${NC}"
echo -e "${GREEN}═══════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Files created in themes/ directory${NC}"
echo "  You can now edit these JSON files and run:"
echo "  make compile-themes"
echo ""
