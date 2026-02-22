-- Create component settings table for theme customization
CREATE TABLE IF NOT EXISTS component_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on category for faster filtering
CREATE INDEX idx_component_settings_category ON component_settings(category);

-- Insert default theme settings
INSERT INTO component_settings (key, value, category, description) VALUES
    -- Colors
    ('color_primary', '#3b82f6', 'colors', 'Primary brand color'),
    ('color_secondary', '#64748b', 'colors', 'Secondary accent color'),
    ('color_tertiary', '#10b981', 'colors', 'Tertiary accent color'),
    
    -- Spacing
    ('spacing_xs', '0.25rem', 'spacing', 'Extra small spacing'),
    ('spacing_sm', '0.5rem', 'spacing', 'Small spacing'),
    ('spacing_md', '1rem', 'spacing', 'Medium spacing'),
    ('spacing_lg', '1.5rem', 'spacing', 'Large spacing'),
    ('spacing_xl', '2rem', 'spacing', 'Extra large spacing'),
    
    -- Border radius
    ('radius_sm', '0.25rem', 'borders', 'Small border radius'),
    ('radius_md', '0.5rem', 'borders', 'Medium border radius'),
    ('radius_lg', '0.75rem', 'borders', 'Large border radius'),
    ('radius_xl', '1rem', 'borders', 'Extra large border radius'),
    
    -- Typography
    ('font_size_base', '1rem', 'typography', 'Base font size'),
    ('font_size_sm', '0.875rem', 'typography', 'Small font size'),
    ('font_size_lg', '1.125rem', 'typography', 'Large font size'),
    ('font_size_xl', '1.25rem', 'typography', 'Extra large font size');
