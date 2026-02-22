use leptos::*;
use leptos_router::*;
use super::ThemeToggle;

#[derive(Debug, Clone, PartialEq)]
pub struct NavItem {
    pub label: String,
    pub path: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavGroup {
    pub title: String,
    pub items: Vec<NavItem>,
}

#[component]
pub fn SideNav(
    groups: Vec<NavGroup>,
) -> impl IntoView {
    view! {
        <nav class="w-64 bg-slate-800 dark:bg-slate-900 text-white min-h-screen p-4 flex flex-col border-r border-slate-700">
            <div class="mb-8">
                <h1 class="text-2xl font-bold" style="color: var(--color-primary);">"Atom Platform"</h1>
                <p class="text-sm text-slate-400">"Admin Panel"</p>
            </div>
            
            <div class="mb-6">
                <ThemeToggle/>
            </div>
            
            <div class="flex-1 space-y-6">
                <For
                    each=move || groups.clone()
                    key=|group| group.title.clone()
                    children=move |group| {
                        view! {
                            <div class="space-y-2">
                                <h3 class="text-xs font-semibold text-slate-400 uppercase tracking-wider px-3">
                                    {group.title}
                                </h3>
                                <For
                                    each=move || group.items.clone()
                                    key=|item| item.path.clone()
                                    children=move |item| {
                                        view! {
                                            <A
                                                href=item.path.clone()
                                                class="nav-item block px-3 py-2 rounded-lg text-sm font-medium hover:bg-slate-700 transition-colors"
                                                active_class="nav-item-active"
                                            >
                                                {item.label}
                                            </A>
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>
        </nav>
    }
}
