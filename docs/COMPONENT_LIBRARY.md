# Component Library Reference

Complete UI component library similar to Bootstrap/Tailwind UI with 20+ production-ready components.

## Table of Contents

- [Buttons](#buttons)
- [Badges](#badges)
- [Form Components](#form-components)
- [Alerts](#alerts)
- [Modal](#modal)
- [Progress](#progress)
- [Spinners](#spinners)
- [Avatars](#avatars)
- [Tables](#tables)
- [DataTable](#datatable)
- [Cards](#cards)
- [Tabs](#tabs)
- [Divider](#divider)
- [Dropdown](#dropdown)
- [Breadcrumbs](#breadcrumbs)
- [Pagination](#pagination)
- [Navigation](#navigation)

---

## Buttons

Versatile button component with 14 variants, 5 sizes, and multiple states.

### Variants

```rust
use crate::components::ui::{Button, ButtonVariant};

// Solid buttons
<Button variant=ButtonVariant::Primary>"Primary"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Tertiary>"Tertiary"</Button>
<Button variant=ButtonVariant::Success>"Success"</Button>
<Button variant=ButtonVariant::Warning>"Warning"</Button>
<Button variant=ButtonVariant::Danger>"Danger"</Button>
<Button variant=ButtonVariant::Info>"Info"</Button>

// Outline buttons
<Button variant=ButtonVariant::OutlinePrimary>"Outline"</Button>
<Button variant=ButtonVariant::OutlineSecondary>"Outline"</Button>
<Button variant=ButtonVariant::OutlineDanger>"Outline"</Button>

// Special variants
<Button variant=ButtonVariant::Ghost>"Ghost"</Button>
<Button variant=ButtonVariant::Link>"Link"</Button>
<Button variant=ButtonVariant::Light>"Light"</Button>
<Button variant=ButtonVariant::Dark>"Dark"</Button>
```

### Sizes

```rust
use crate::components::ui::ButtonSize;

<Button size=ButtonSize::Xs>"Extra Small"</Button>  // 10px padding
<Button size=ButtonSize::Sm>"Small"</Button>        // 12px padding
<Button size=ButtonSize::Md>"Medium"</Button>       // 16px padding (default)
<Button size=ButtonSize::Lg>"Large"</Button>        // 20px padding
<Button size=ButtonSize::Xl>"Extra Large"</Button>  // 24px padding
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `ButtonVariant` | `Primary` | Visual style |
| `size` | `ButtonSize` | `Md` | Size variation |
| `disabled` | `bool` | `false` | Disabled state |
| `full_width` | `bool` | `false` | Full width (w-full) |
| `on_click` | `Option<Callback>` | `None` | Click handler |
| `class` | `String` | `""` | Additional CSS classes |

---

## Badges

Small status indicators with 9 variants and 3 sizes.

### Usage

```rust
use crate::components::ui::{Badge, BadgeVariant, BadgeSize};

// Variants
<Badge variant=BadgeVariant::Primary>"Primary"</Badge>
<Badge variant=BadgeVariant::Success>"Success"</Badge>
<Badge variant=BadgeVariant::Warning>"Warning"</Badge>
<Badge variant=BadgeVariant::Danger>"Danger"</Badge>

// Sizes
<Badge size=BadgeSize::Sm>"Small"</Badge>
<Badge size=BadgeSize::Md>"Medium"</Badge>
<Badge size=BadgeSize::Lg>"Large"</Badge>

// Pill shape
<Badge pill=true>"Pill Badge"</Badge>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `BadgeVariant` | `Primary` | Color scheme |
| `size` | `BadgeSize` | `Md` | Size variation |
| `pill` | `bool` | `false` | Fully rounded (pill shape) |
| `class` | `String` | `""` | Additional CSS classes |

---

## Form Components

### Input

Text input with validation states and sizes.

```rust
use crate::components::ui::{Input, InputSize, InputVariant};

// Basic input
<Input placeholder="Enter text...".to_string()/>

// With validation
<Input variant=InputVariant::Success/>
<Input variant=InputVariant::Warning/>
<Input variant=InputVariant::Error/>

// Sizes
<Input size=InputSize::Sm/>
<Input size=InputSize::Lg/>
```

### Textarea

Multi-line text input.

```rust
use crate::components::ui::Textarea;

<Textarea placeholder="Enter message...".to_string() rows=5/>
```

### FormGroup

Wrapper with label, help text, and error messages.

```rust
use crate::components::ui::FormGroup;

<FormGroup 
    label="Email".to_string() 
    required=true
    help_text="We'll never share your email.".to_string()
>
    <Input input_type="email".to_string()/>
</FormGroup>

// With error
<FormGroup 
    label="Password".to_string()
    error="Password too weak".to_string()
>
    <Input input_type="password".to_string() variant=InputVariant::Error/>
</FormGroup>
```

---

## Alerts

Contextual feedback messages.

```rust
use crate::components::ui::{Alert, AlertVariant};

// With title
<Alert variant=AlertVariant::Success title="Success!".to_string()>
    "Operation completed successfully."
</Alert>

// Dismissible
<Alert 
    variant=AlertVariant::Warning
    on_close=Some(Callback::new(|_| { /* close logic */ }))
>
    "Warning message"
</Alert>

// Variants: Primary, Success, Warning, Danger, Info
```

---

## Modal

Overlay dialog component.

```rust
use crate::components::ui::{Modal, ModalSize};

let (open, set_open) = create_signal(false);

<Modal
    open=Signal::derive(move || open.get())
    on_close=Some(Callback::new(move |_| set_open.set(false)))
    title="Confirm Action".to_string()
    size=ModalSize::Md
>
    <p>"Are you sure you want to proceed?"</p>
</Modal>

// Sizes: Sm, Md, Lg, Xl, Full
```

---

## Progress

Progress bars with variants and sizes.

```rust
use crate::components::ui::{Progress, ProgressVariant, ProgressSize};

// Basic
<Progress value=Signal::derive(move || 75.0)/>

// With label
<Progress 
    value=Signal::derive(move || 60.0)
    show_label=true
/>

// Variants
<Progress value=... variant=ProgressVariant::Success/>
<Progress value=... variant=ProgressVariant::Warning/>
<Progress value=... variant=ProgressVariant::Danger/>

// Sizes: Sm (1px), Md (2px), Lg (4px)
```

---

## Spinners

Loading indicators.

```rust
use crate::components::ui::{Spinner, SpinnerOverlay, SpinnerSize};

// Basic spinner
<Spinner size=SpinnerSize::Md/>

// Full page overlay
<SpinnerOverlay 
    show=loading.get()
    message="Loading data...".to_string()
/>

// Sizes: Sm, Md, Lg, Xl
```

---

## Avatars

User profile images or initials.

```rust
use crate::components::ui::{Avatar, AvatarSize};

// With initials
<Avatar initials="JD".to_string() size=AvatarSize::Md/>

// With image
<Avatar src="/avatar.jpg".to_string() alt="John Doe".to_string()/>

// Rounded (square with rounded corners)
<Avatar initials="AB".to_string() rounded=true/>

// Sizes: Xs, Sm, Md, Lg, Xl
```

---

## Tables

### Simple Table

Traditional table components for manual control.

```rust
use crate::components::ui::{Table, TableHeader, TableBody, TableRow, TableHeaderCell, TableCell};

<Table>
    <TableHeader>
        <TableRow>
            <TableHeaderCell>"Name"</TableHeaderCell>
            <TableHeaderCell>"Email"</TableHeaderCell>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>"John Doe"</TableCell>
            <TableCell>"john@example.com"</TableCell>
        </TableRow>
    </TableBody>
</Table>
```

### DataTable

Feature-rich declarative table (recommended for most use cases).

```rust
use crate::components::ui::{DataTable, Column};

let columns = vec![
    Column::new("Name", |user: &User| {
        view! { <span>{user.name.clone()}</span> }.into_view()
    }).with_width("40%"),
    
    Column::new("Email", |user: &User| {
        view! { <span>{user.email.clone()}</span> }.into_view()
    }).with_width("40%"),
    
    Column::new("Actions", |user: &User| {
        view! {
            <button>"Edit"</button>
        }.into_view()
    }).with_width("20%"),
];

<DataTable
    data=Signal::derive(move || users.get())
    columns=columns
    key_fn=|user: &User| user.id.to_string()
    loading=loading.get()
    empty_message="No users found".to_string()
/>
```

---

## Cards

Container component with optional title.

```rust
use crate::components::ui::Card;

// Simple card
<Card>
    "Card content here"
</Card>

// With title
<Card title="Card Title".to_string()>
    "Card content"
</Card>

// Custom styling
<Card title="Custom".to_string() class="border-l-4 border-blue-500">
    "Content"
</Card>
```

---

## Tabs

Tabbed interface component.

```rust
use crate::components::ui::{TabList, Tab, TabPanel};

let (active, set_active) = create_signal(0);

<div>
    <TabList>
        <Tab 
            active=active.get() == 0
            on_click=Some(Callback::new(move |_| set_active.set(0)))
        >
            "Tab 1"
        </Tab>
        <Tab 
            active=active.get() == 1
            on_click=Some(Callback::new(move |_| set_active.set(1)))
        >
            "Tab 2"
        </Tab>
    </TabList>
    
    <TabPanel active=active.get() == 0>
        "Content for tab 1"
    </TabPanel>
    <TabPanel active=active.get() == 1>
        "Content for tab 2"
    </TabPanel>
</div>
```

---

## Divider

Visual separator with optional label.

```rust
use crate::components::ui::{Divider, DividerOrientation};

// Horizontal divider
<Divider/>

// With label
<Divider label="OR".to_string()/>

// Vertical
<Divider orientation=DividerOrientation::Vertical/>
```

---

## Dropdown

Dropdown menu component.

```rust
use crate::components::ui::{Dropdown, DropdownItem, DropdownDivider};

<Dropdown label="Actions".to_string()>
    <DropdownItem>"Edit"</DropdownItem>
    <DropdownItem>"Duplicate"</DropdownItem>
    <DropdownDivider/>
    <DropdownItem danger=true>"Delete"</DropdownItem>
</Dropdown>
```

---

## Breadcrumbs

Navigation breadcrumb trail.

```rust
use crate::components::ui::{Breadcrumbs, BreadcrumbItem};

<Breadcrumbs items=vec![
    BreadcrumbItem { label: "Home".to_string(), href: Some("/".to_string()) },
    BreadcrumbItem { label: "Products".to_string(), href: Some("/products".to_string()) },
    BreadcrumbItem { label: "Item".to_string(), href: None },
]/>
```

---

## Pagination

Page navigation component.

```rust
use crate::components::ui::Pagination;

let (page, set_page) = create_signal(0);

<Pagination
    current_page=Signal::derive(move || page.get())
    total_pages=20_usize
    on_page_change=Callback::new(move |new_page| set_page.set(new_page))
    max_visible=5_usize
/>
```

---

## Navigation

### SideNav

Sidebar navigation with grouped items.

```rust
use crate::components::ui::sidenav::{SideNav, NavGroup, NavItem};

let groups = vec![
    NavGroup {
        title: "Main".to_string(),
        items: vec![
            NavItem {
                label: "Dashboard".to_string(),
                path: "/".to_string(),
                icon: None,
            },
        ],
    },
];

<SideNav groups=groups/>
```

### ThemeToggle

Quick theme switcher (Light/Dark).

```rust
use crate::components::ui::ThemeToggle;

<ThemeToggle/>  // Shows 🌙 or ☀️ icon
```

---

## Theme Integration

All components support:
- **Dark mode** (automatic via `.dark` class on `<html>`)
- **Custom colors** (Primary, Secondary, Tertiary via CSS variables)
- **Responsive design** (Mobile-first with Tailwind breakpoints)
- **Accessibility** (ARIA attributes, keyboard navigation)

### CSS Variables

Components use theme CSS variables:
- `--color-primary` - Main brand color
- `--color-secondary` - Accent color
- `--color-tertiary` - Success/highlight color
- `--bg-primary` - Main background
- `--bg-secondary` - Card/component backgrounds
- `--text-primary` - Primary text color
- `--text-secondary` - Muted text color

---

## Component Showcase

Visit `/showcase` in the application to see all components with live examples.

**Navigation**: Developer > Components

The showcase includes:
- All button variants and sizes
- Badge examples
- Form components with validation
- Alert types
- Progress bars
- Spinners
- Avatars
- Pagination
- Breadcrumbs
- Dividers
- Dropdowns

---

## Best Practices

### Consistent Spacing

Use Tailwind's spacing scale:
- `gap-2` / `space-y-2` - 0.5rem
- `gap-4` / `space-y-4` - 1rem (default)
- `gap-6` / `space-y-6` - 1.5rem
- `gap-8` / `space-y-8` - 2rem

### Color Usage

- **Primary**: Main CTAs, brand elements
- **Secondary**: Less important actions
- **Tertiary**: Success states, positive actions
- **Success**: Confirmations, completed states
- **Warning**: Cautions, pending states
- **Danger**: Destructive actions, errors
- **Info**: Informational messages

### Responsive Design

All components are mobile-first and responsive:
```rust
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    // Auto-responsive grid
</div>
```

### Dark Mode

Components automatically adapt when `.dark` class is on `<html>`:
- No additional code needed
- All colors adjust
- Shadows adapted for dark backgrounds
- Border colors optimized

---

## Creating Custom Components

Follow this pattern:

```rust
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MyComponentVariant {
    Primary,
    Secondary,
}

#[component]
pub fn MyComponent(
    children: Children,
    #[prop(optional, default = MyComponentVariant::Primary)] variant: MyComponentVariant,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "base classes here";
    let variant_classes = match variant {
        MyComponentVariant::Primary => "primary classes",
        MyComponentVariant::Secondary => "secondary classes",
    };
    
    view! {
        <div class=format!("{} {} {}", base_classes, variant_classes, class)>
            {children()}
        </div>
    }
}
```

---

**Total Components**: 20+
**Variants**: 50+
**Props**: 100+

For live examples, visit: `http://localhost:8081/showcase`
