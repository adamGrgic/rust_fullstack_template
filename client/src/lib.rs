mod api;
mod components;
mod pages;
mod theme;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

use components::ui::sidenav::{SideNav, NavGroup, NavItem};
use pages::{HomePage, AdminTodosPage, SettingsThemePage, ComponentShowcasePage};
use theme::ThemeContext;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize theme context (loads from localStorage immediately)
    let theme = ThemeContext::new();
    provide_context(theme.clone());
    
    // Apply cached theme immediately to prevent flash
    theme.apply_to_document();

    // Then fetch fresh settings from API in background
    let theme_clone = theme.clone();
    create_effect(move |_| {
        let theme = theme_clone.clone();
        spawn_local(async move {
            match api::fetch_settings().await {
                Ok(settings) => {
                    theme.load_from_api(settings);
                    theme.apply_to_document();
                }
                Err(e) => {
                    log::error!("Failed to load theme settings: {}", e);
                }
            }
        });
    });

    let nav_groups = vec![
        NavGroup {
            title: "Main".to_string(),
            items: vec![
                NavItem {
                    label: "Home".to_string(),
                    path: "/".to_string(),
                    icon: None,
                },
                NavItem {
                    label: "Admin Todos".to_string(),
                    path: "/admin/todos".to_string(),
                    icon: None,
                },
            ],
        },
        NavGroup {
            title: "Settings".to_string(),
            items: vec![
                NavItem {
                    label: "Theme".to_string(),
                    path: "/settings/theme".to_string(),
                    icon: None,
                },
            ],
        },
        NavGroup {
            title: "Developer".to_string(),
            items: vec![
                NavItem {
                    label: "Components".to_string(),
                    path: "/showcase".to_string(),
                    icon: None,
                },
            ],
        },
    ];

    view! {
        <Title text="Atom Platform - Admin"/>
        <Router>
            <div class="flex min-h-screen">
                <SideNav groups=nav_groups/>
                <main class="flex-1 p-8 bg-slate-50">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/admin/todos" view=AdminTodosPage/>
                        <Route path="/settings/theme" view=SettingsThemePage/>
                        <Route path="/showcase" view=ComponentShowcasePage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(|| view! { <App/> })
}
