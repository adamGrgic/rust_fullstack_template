#!/bin/bash

# Atom Platform Theme Compiler
# Compiles JSON theme files to database

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

THEMES_DIR="themes"
DB_NAME="atom"
DB_USER="atom"
DB_PASSWORD="atom"

echo -e "${CYAN}🎨 Atom Platform Theme Compiler${NC}"
echo -e "${CYAN}═══════════════════════════════${NC}"
echo ""

# Check if themes directory exists
if [ ! -d "$THEMES_DIR" ]; then
    echo -e "${RED}❌ Themes directory not found: $THEMES_DIR${NC}"
    exit 1
fi

# Check if PostgreSQL is available
if ! command -v psql &> /dev/null; then
    echo -e "${RED}❌ PostgreSQL client (psql) not found${NC}"
    exit 1
fi

# Count JSON files
THEME_COUNT=$(ls -1 "$THEMES_DIR"/*.json 2>/dev/null | wc -l)

if [ "$THEME_COUNT" -eq 0 ]; then
    echo -e "${YELLOW}⚠️  No theme JSON files found in $THEMES_DIR${NC}"
    exit 0
fi

echo -e "${BLUE}📂 Found $THEME_COUNT theme file(s)${NC}"
echo ""

# Process each theme file
for theme_file in "$THEMES_DIR"/*.json; do
    if [ ! -f "$theme_file" ]; then
        continue
    fi
    
    filename=$(basename "$theme_file")
    echo -e "${CYAN}Processing: $filename${NC}"
    
    # Use Python to parse JSON and generate SQL
    PGPASSWORD=$DB_PASSWORD python3 - "$theme_file" <<'PYTHON_SCRIPT'
import json
import sys
import subprocess

theme_file = sys.argv[1]

with open(theme_file, 'r') as f:
    theme_data = json.load(f)

theme_name = theme_data['name']
is_active = theme_data.get('is_active', False)

print(f"  Theme: {theme_name}")
print(f"  Active: {is_active}")

# Create SQL commands
sql_commands = []

# Check if theme exists and get/create ID
sql_commands.append(f"""
DO $$
DECLARE
    theme_uuid UUID;
    setting_count INTEGER;
BEGIN
    -- Get or create theme
    SELECT id INTO theme_uuid FROM themes WHERE name = '{theme_name}';
    
    IF theme_uuid IS NULL THEN
        INSERT INTO themes (name, is_active)
        VALUES ('{theme_name}', {str(is_active).lower()})
        RETURNING id INTO theme_uuid;
        RAISE NOTICE 'Created theme: {theme_name}';
    ELSE
        -- Update theme metadata
        UPDATE themes SET is_active = {str(is_active).lower()} WHERE id = theme_uuid;
        RAISE NOTICE 'Updated theme: {theme_name}';
    END IF;
    
    -- Delete old settings
    DELETE FROM theme_settings WHERE theme_id = theme_uuid;
    
    -- Insert new settings
""")

# Process settings by category
settings_insert = []
for category, settings in theme_data['settings'].items():
    for key, setting_data in settings.items():
        value = setting_data['value']
        description = setting_data.get('description', '')
        settings_insert.append(
            f"    INSERT INTO theme_settings (theme_id, key, value, category, description) "
            f"VALUES (theme_uuid, '{key}', '{value}', '{category}', '{description}');"
        )

sql_commands.append('\n'.join(settings_insert))
sql_commands.append(f"""
    
    SELECT COUNT(*) INTO setting_count FROM theme_settings WHERE theme_id = theme_uuid;
    RAISE NOTICE 'Imported % settings', setting_count;
END $$;
""")

# Execute SQL
full_sql = '\n'.join(sql_commands)

try:
    result = subprocess.run(
        ['psql', '-U', 'atom', '-d', 'atom', '-h', 'localhost'],
        input=full_sql,
        capture_output=True,
        text=True,
        env={'PGPASSWORD': 'atom'}
    )
    
    # Print notices (these contain our RAISE NOTICE messages)
    if result.stderr:
        for line in result.stderr.split('\n'):
            if 'NOTICE:' in line:
                notice = line.split('NOTICE:')[1].strip()
                print(f"  ✓ {notice}")
    
    if result.returncode == 0:
        print(f"  {chr(10003)} Success")
    else:
        print(f"  ✗ Error: {result.stderr}")
        sys.exit(1)
        
except Exception as e:
    print(f"  ✗ Error: {e}")
    sys.exit(1)

PYTHON_SCRIPT
    
    echo ""
done

echo -e "${GREEN}═══════════════════════════════${NC}"
echo -e "${GREEN}✅ Theme compilation complete!${NC}"
echo -e "${GREEN}═══════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Restart the application: make stop && make dev"
echo "  2. Visit /settings/theme to see your themes"
echo ""
