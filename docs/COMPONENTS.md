# Component System Documentation

This document describes the reusable UI component system and theme customization features.

## Overview

The Atom Platform includes a comprehensive component library with live customization capabilities. The system supports:

- **Reusable UI Components**: DataTable, Table, Card, Button with consistent styling
- **Dark/Light Mode**: Complete theme switching with persistent preferences
- **Theme Customization**: Primary, Secondary, and Tertiary color schemes
- **Live Settings**: Database-backed configuration that can be adjusted in real-time
- **Admin Layout**: Professional sidenav with organized navigation and theme toggle

## Architecture

### Theme System

The application features a comprehensive theme system with:

#### Theme Manager

The application features a sophisticated tab-based theme manager:

- **Light Theme** (default): Clean, bright interface with blue accents
- **Dark Theme**: Darker colors optimized for low-light, lighter blue (#60a5fa)
- **Custom Themes**: Create unlimited custom themes with "+" button
- **Theme Tabs**: Switch between themes instantly
- **Quick Toggle**: Sidebar button to switch between Light/Dark (🌙/☀️)
- **Rename Themes**: Edit names of custom themes
- **Activate Theme**: Set any theme as active
- **Persistent**: All saved to database and cached in localStorage

#### Color System

The application uses a **three-color theme system**:

- **Primary**: Main brand color (default: Blue #3b82f6)
- **Secondary**: Accent color (default: Slate #64748b)
- **Tertiary**: Success/highlight color (default: Emerald #10b981)

All colors work in both light and dark modes.

### Database Schema

Themes and settings are stored in PostgreSQL:

```sql
themes
├── id (UUID)
├── name (TEXT, UNIQUE)
├── is_active (BOOLEAN)       -- Only one theme can be active
├── created_at (TIMESTAMPTZ)
└── updated_at (TIMESTAMPTZ)

theme_settings
├── id (UUID)
├── theme_id (UUID)           -- FK to themes
├── key (TEXT)                -- e.g., "color_primary"
├── value (TEXT)              -- e.g., "#3b82f6"
├── category (TEXT)           -- e.g., "colors", "spacing", "borders"
├── description (TEXT)
├── created_at (TIMESTAMPTZ)
├── updated_at (TIMESTAMPTZ)
└── UNIQUE(theme_id, key)
```

**Default Themes:**
- **Light**: Traditional light mode with blue (#3b82f6) primary
- **Dark**: Dark mode with lighter blue (#60a5fa) for contrast

**Setting Categories:**
- **colors**: Primary, secondary, tertiary colors
- **spacing**: XS, SM, MD, LG, XL spacing values
- **borders**: Border radius values (SM, MD, LG, XL)
- **typography**: Font sizes

## Component Library

### ThemeToggle

Theme switcher button (already integrated in SideNav):

```rust
use crate::components::ui::ThemeToggle;

<ThemeToggle/>
```

Features:
- Toggles between light and dark mode
- Shows moon (🌙) in light mode, sun (☀️) in dark mode
- Saves preference to database and localStorage
- Applies changes instantly across the app

### DataTable

Feature-rich table component for displaying structured data:

```rust
use crate::components::ui::{DataTable, Column};

let columns = vec![
    Column::new("Name", |item: &MyType| {
        view! { <span>{item.name.clone()}</span> }.into_view()
    }).with_width("40%"),
    
    Column::new("Status", |item: &MyType| {
        view! { <StatusBadge status=item.status/> }.into_view()
    }).with_width("20%"),
];

<DataTable
    data=Signal::derive(move || my_data.get())
    columns=columns
    key_fn=|item: &MyType| item.id.to_string()
    loading=loading.get()
    empty_message="No data available".to_string()
/>
```

Features:
- Declarative column definitions
- Built-in loading and empty states
- Column width control
- Hover effects
- Type-safe and reusable

### Button

Reusable button component with variants:

```rust
use crate::components::ui::Button;

<Button
    variant=ButtonVariant::Primary
    on_click=Some(callback)
    disabled=false
>
    "Click Me"
</Button>
```

**Variants:**
- `Primary` - Blue (main actions)
- `Secondary` - Slate (secondary actions)
- `Tertiary` - Emerald (success actions)
- `Danger` - Red (destructive actions)

### Card

Container component with optional title:

```rust
use crate::components::ui::Card;

<Card title="My Card Title".to_string()>
    // Card content here
</Card>
```

### Table

Professional table components:

```rust
use crate::components::ui::{Table, TableHeader, TableBody, TableRow, TableHeaderCell, TableCell};

<Table>
    <TableHeader>
        <TableRow>
            <TableHeaderCell>"Column 1"</TableHeaderCell>
            <TableHeaderCell>"Column 2"</TableHeaderCell>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>"Data 1"</TableCell>
            <TableCell>"Data 2"</TableCell>
        </TableRow>
    </TableBody>
</Table>
```

### SideNav

Navigation sidebar with grouped menu items:

```rust
use crate::components::ui::sidenav::{SideNav, NavGroup, NavItem};

let nav_groups = vec![
    NavGroup {
        title: "Main".to_string(),
        items: vec![
            NavItem {
                label: "Home".to_string(),
                path: "/".to_string(),
                icon: None,
            },
        ],
    },
];

<SideNav groups=nav_groups/>
```

## Page Views

### Home Page (`/`)

Traditional card-based todo view with create form and individual todo cards.

**Features:**
- Create new todos
- Edit todos inline
- Toggle status
- Delete todos
- Card layout with status indicators

### Admin Todos (`/admin/todos`)

Professional table view for managing todos at scale.

**Features:**
- Tabular data display
- Status badges
- Quick delete actions
- Sortable columns
- Responsive design

### Settings > Components (`/settings/components`)

Live theme customization interface.

**Features:**
- Edit colors with color picker
- Adjust spacing values
- Modify border radius
- Real-time updates
- Organized by category (Colors, Spacing, Borders, Typography)
- Color preview swatches

## API Endpoints

### Component Settings

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/settings` | List all component settings |
| GET | `/api/settings/:id` | Get single setting |
| PUT | `/api/settings/:id` | Update setting value |

**Example Request:**
```bash
curl -X PUT http://localhost:8080/api/settings/{id} \
  -H "Content-Type: application/json" \
  -d '{"value": "#ff0000"}'
```

## Usage Examples

### Adding a New Color

1. **Database**: Insert new setting
   ```sql
   INSERT INTO component_settings (key, value, category, description)
   VALUES ('color_accent', '#a855f7', 'colors', 'Accent color for highlights');
   ```

2. **Use in Components**:
   ```rust
   // Fetch from settings and apply dynamically
   ```

### Creating a New Component

1. Create file in `client/src/components/ui/`
2. Export from `client/src/components/ui/mod.rs`
3. Use Tailwind utility classes
4. Make it configurable via props

**Example:**
```rust
#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span class=format!("inline-block px-3 py-1 rounded-full text-xs font-semibold {}", class)>
            {children()}
        </span>
    }
}
```

### Adding a New Page

1. Create file in `client/src/pages/`
2. Export from `client/src/pages/mod.rs`
3. Add route in `client/src/lib.rs`
4. Add nav item to sidenav groups

## Styling Guidelines

### Use Tailwind Utilities

Prefer Tailwind classes over custom CSS:

✅ **Good:**
```rust
view! {
    <div class="flex items-center gap-4 p-6 bg-white rounded-lg shadow-md">
        "Content"
    </div>
}
```

❌ **Avoid:**
```css
.my-custom-class {
    display: flex;
    align-items: center;
    /* ... */
}
```

### Consistent Spacing

Use the spacing scale:
- `gap-2` (0.5rem) - Tight spacing
- `gap-4` (1rem) - Default spacing
- `gap-6` (1.5rem) - Comfortable spacing
- `gap-8` (2rem) - Generous spacing

### Color Usage

- **Primary (Blue)**: Main actions, primary buttons, brand elements
- **Secondary (Slate)**: Secondary actions, muted elements
- **Tertiary (Emerald)**: Success states, positive actions
- **Semantic Colors**: Red (danger/delete), Amber (warning/in-progress)

## Future Enhancements

Potential additions to the component system:

1. **More Components**
   - Modal/Dialog
   - Dropdown menus
   - Toast notifications
   - Form inputs (with validation)
   - Tabs
   - Accordion

2. **Advanced Theming**
   - Dark mode support
   - Multiple theme presets
   - Export/import themes
   - Theme preview mode

3. **Component Playground**
   - Live code editor
   - Component previews
   - Props documentation
   - Copy code snippets

4. **Accessibility**
   - ARIA labels
   - Keyboard navigation
   - Focus management
   - Screen reader support

## Troubleshooting

### Settings Not Loading

Check the API endpoint:
```bash
curl http://localhost:8080/api/settings
```

Verify migration ran:
```bash
PGPASSWORD=atom psql -U atom -d atom -h localhost \
  -c "SELECT * FROM component_settings LIMIT 5;"
```

### Styles Not Updating

1. Rebuild Tailwind: `cd client && ./tailwindcss -i ./input.css -o ./style.css --minify`
2. Restart client: `make stop && make dev`
3. Hard refresh browser: `Ctrl+Shift+R` or `Cmd+Shift+R`

### Component Not Rendering

1. Check browser console for errors (F12)
2. Check client logs: `tail -f .logs/client.log`
3. Verify component is exported from `mod.rs`
4. Check Tailwind classes are generated in `style.css`

---

**Last Updated:** 2026-02-22  
**Tailwind Version:** v4.2.0  
**Leptos Version:** 0.6
