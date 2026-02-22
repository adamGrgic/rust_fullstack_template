-- Create themes table
CREATE TABLE IF NOT EXISTS themes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create theme_settings table (component settings per theme)
CREATE TABLE IF NOT EXISTS theme_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    theme_id UUID NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(theme_id, key)
);

-- Create indexes
CREATE INDEX idx_theme_settings_theme_id ON theme_settings(theme_id);
CREATE INDEX idx_theme_settings_category ON theme_settings(theme_id, category);

-- Insert default Light theme
INSERT INTO themes (name, is_active) VALUES ('Light', true);

-- Insert default Dark theme
INSERT INTO themes (name, is_active) VALUES ('Dark', false);

-- Get the theme IDs
DO $$
DECLARE
    light_theme_id UUID;
    dark_theme_id UUID;
BEGIN
    SELECT id INTO light_theme_id FROM themes WHERE name = 'Light';
    SELECT id INTO dark_theme_id FROM themes WHERE name = 'Dark';
    
    -- Copy existing settings to Light theme
    INSERT INTO theme_settings (theme_id, key, value, category, description)
    SELECT light_theme_id, key, value, category, description
    FROM component_settings;
    
    -- Create Dark theme settings with adjusted colors
    INSERT INTO theme_settings (theme_id, key, value, category, description) VALUES
        -- Dark mode colors
        (dark_theme_id, 'color_primary', '#60a5fa', 'colors', 'Primary brand color (lighter for dark mode)'),
        (dark_theme_id, 'color_secondary', '#94a3b8', 'colors', 'Secondary accent color'),
        (dark_theme_id, 'color_tertiary', '#34d399', 'colors', 'Tertiary accent color'),
        
        -- Spacing (same as light)
        (dark_theme_id, 'spacing_xs', '0.25rem', 'spacing', 'Extra small spacing'),
        (dark_theme_id, 'spacing_sm', '0.5rem', 'spacing', 'Small spacing'),
        (dark_theme_id, 'spacing_md', '1rem', 'spacing', 'Medium spacing'),
        (dark_theme_id, 'spacing_lg', '1.5rem', 'spacing', 'Large spacing'),
        (dark_theme_id, 'spacing_xl', '2rem', 'spacing', 'Extra large spacing'),
        
        -- Border radius (same as light)
        (dark_theme_id, 'radius_sm', '0.25rem', 'borders', 'Small border radius'),
        (dark_theme_id, 'radius_md', '0.5rem', 'borders', 'Medium border radius'),
        (dark_theme_id, 'radius_lg', '0.75rem', 'borders', 'Large border radius'),
        (dark_theme_id, 'radius_xl', '1rem', 'borders', 'Extra large border radius'),
        
        -- Typography (same as light)
        (dark_theme_id, 'font_size_base', '1rem', 'typography', 'Base font size'),
        (dark_theme_id, 'font_size_sm', '0.875rem', 'typography', 'Small font size'),
        (dark_theme_id, 'font_size_lg', '1.125rem', 'typography', 'Large font size'),
        (dark_theme_id, 'font_size_xl', '1.25rem', 'typography', 'Extra large font size');
END $$;

-- Drop old component_settings table and theme_mode from it
-- (We're moving to the new themes system)
-- Keep component_settings for now for backwards compatibility, but mark as deprecated
UPDATE component_settings SET description = 'DEPRECATED - Use themes system' WHERE key = 'theme_mode';
