use leptos::*;
use platform_core::{Theme, ThemeWithSettings, ThemeCreate, ThemeUpdate, ComponentSetting};

use crate::api;
use crate::components::ui::{Card, TabList, Tab, TabPanel};
use crate::theme::ThemeContext;

#[component]
pub fn SettingsThemePage() -> impl IntoView {
    let (themes, set_themes) = create_signal(Vec::<Theme>::new());
    let (active_tab, set_active_tab) = create_signal(0_usize);
    let (loading, set_loading) = create_signal(true);
    let (creating_theme, set_creating_theme) = create_signal(false);

    // Load themes on mount
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match api::fetch_themes().await {
                Ok(fetched_themes) => {
                    set_themes.set(fetched_themes);
                }
                Err(e) => {
                    log::error!("Failed to load themes: {}", e);
                }
            }
            set_loading.set(false);
        });
    });

    let handle_create_theme = move |_| {
        set_creating_theme.set(true);
        spawn_local(async move {
            let new_theme = ThemeCreate {
                name: "Untitled Theme".to_string(),
            };

            match api::create_theme(new_theme).await {
                Ok(theme) => {
                    set_themes.update(|themes| themes.push(theme));
                    set_active_tab.set(themes.get().len() - 1);
                }
                Err(e) => {
                    log::error!("Failed to create theme: {}", e);
                }
            }
            set_creating_theme.set(false);
        });
    };

    view! {
        <div>
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-slate-800 dark:text-slate-100 mb-2">"Theme Manager"</h1>
                <p class="text-slate-600 dark:text-slate-400">"Manage light, dark, and custom themes with live preview"</p>
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="text-center p-8 bg-white dark:bg-slate-800 rounded-xl shadow-md">
                            "Loading themes..."
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <TabList>
                                <For
                                    each=move || themes.get()
                                    key=|theme| theme.id
                                    children=move |theme| {
                                        let idx = themes.get().iter().position(|t| t.id == theme.id).unwrap_or(0);
                                        let theme_name = theme.name.clone();
                                        view! {
                                            <button
                                                class=move || if active_tab.get() == idx {
                                                    "px-4 py-2 font-medium text-sm transition-all cursor-pointer border-b-2 text-slate-900 dark:text-slate-100"
                                                } else {
                                                    "px-4 py-2 font-medium text-sm transition-all cursor-pointer border-b-2 border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-600"
                                                }
                                                style=move || if active_tab.get() == idx { "border-color: var(--color-primary);" } else { "" }
                                                on:click=move |_| set_active_tab.set(idx)
                                            >
                                                {theme_name}
                                            </button>
                                        }
                                    }
                                />
                                <button
                                    on:click=handle_create_theme
                                    disabled=creating_theme.get()
                                    class="px-4 py-2 font-medium text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 transition-colors disabled:opacity-50"
                                    title="Create new theme"
                                >
                                    "+"
                                </button>
                            </TabList>

                            <For
                                each=move || themes.get()
                                key=|theme| theme.id
                                children=move |theme| {
                                    let idx = themes.get().iter().position(|t| t.id == theme.id).unwrap_or(0);
                                    let theme_id = theme.id;
                                    let theme_name = theme.name.clone();
                                    let is_active = theme.is_active;
                                    view! {
                                        <div class=move || if active_tab.get() == idx { "block" } else { "hidden" }>
                                            <ThemeEditor theme_id=theme_id theme_name=theme_name is_active=is_active/>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn ThemeEditor(
    theme_id: uuid::Uuid,
    theme_name: String,
    is_active: bool,
) -> impl IntoView {
    let (settings, set_settings) = create_signal(Vec::<ComponentSetting>::new());
    let (loading, set_loading) = create_signal(true);
    let (editing_name, set_editing_name) = create_signal(false);
    let (new_name, set_new_name) = create_signal(theme_name.clone());

    // Load theme settings
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match api::fetch_theme_with_settings(theme_id).await {
                Ok(theme_with_settings) => {
                    set_settings.set(theme_with_settings.settings);
                }
                Err(e) => {
                    log::error!("Failed to load theme settings: {}", e);
                }
            }
            set_loading.set(false);
        });
    });

    let handle_activate = move |_| {
        spawn_local(async move {
            let update = ThemeUpdate {
                name: None,
                is_active: Some(true),
            };

            match api::update_theme(theme_id, update).await {
                Ok(_) => {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                }
                Err(e) => {
                    log::error!("Failed to activate theme: {}", e);
                }
            }
        });
    };

    let handle_save_name = move |_| {
        let name = new_name.get();
        spawn_local(async move {
            let update = ThemeUpdate {
                name: Some(name),
                is_active: None,
            };

            match api::update_theme(theme_id, update).await {
                Ok(_) => {
                    set_editing_name.set(false);
                }
                Err(e) => {
                    log::error!("Failed to update theme name: {}", e);
                }
            }
        });
    };

    let grouped_settings = move || {
        let all_settings = settings.get();
        let mut groups: std::collections::HashMap<String, Vec<ComponentSetting>> = std::collections::HashMap::new();
        
        for setting in all_settings {
            groups.entry(setting.category.clone())
                .or_insert_with(Vec::new)
                .push(setting);
        }
        
        let mut sorted_groups: Vec<_> = groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
        sorted_groups
    };

    view! {
        <div class="space-y-6">
            <div class="flex items-center gap-4 p-4 bg-white dark:bg-slate-800 rounded-lg shadow-md">
                {move || if editing_name.get() {
                    view! {
                        <div class="flex-1 flex gap-2">
                            <input
                                type="text"
                                prop:value=new_name
                                on:input=move |ev| set_new_name.set(event_target_value(&ev))
                                class="flex-1 px-3 py-2 border-2 border-slate-200 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                                placeholder="Theme name"
                            />
                            <button
                                on:click=handle_save_name
                                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                            >
                                "Save"
                            </button>
                            <button
                                on:click=move |_| set_editing_name.set(false)
                                class="px-4 py-2 bg-slate-600 text-white rounded-lg hover:bg-slate-700 transition-colors"
                            >
                                "Cancel"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="flex-1 flex items-center gap-4">
                            <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-100">{theme_name.clone()}</h2>
                            {if !is_active {
                                view! {
                                    <button
                                        on:click=move |_| set_editing_name.set(true)
                                        class="text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200"
                                    >
                                        "Rename"
                                    </button>
                                }.into_view()
                            } else {
                                view! { <div/> }.into_view()
                            }}
                        </div>
                        {if is_active {
                            view! {
                                <span class="px-3 py-1 bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 rounded-full text-xs font-semibold">
                                    "Active"
                                </span>
                            }.into_view()
                        } else {
                            view! {
                                <button
                                    on:click=handle_activate
                                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                                >
                                    "Activate Theme"
                                </button>
                            }.into_view()
                        }}
                    }.into_view()
                }}
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="text-center p-8">"Loading settings..."</div>
                    }.into_view()
                } else {
                    view! {
                        <div class="space-y-6">
                            <For
                                each=grouped_settings
                                key=|(category, _)| category.clone()
                                children=move |(category, group_settings)| {
                                    let card_title = category.clone().replace("_", " ").to_uppercase();
                                    view! {
                                        <Card title=card_title>
                                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                                <For
                                                    each=move || group_settings.clone()
                                                    key=|setting| setting.id
                                                    children=move |setting| {
                                                        view! {
                                                            <ThemeSettingItem 
                                                                theme_id=theme_id
                                                                setting=setting
                                                            />
                                                        }
                                                    }
                                                />
                                            </div>
                                        </Card>
                                    }
                                }
                            />
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn ThemeSettingItem(
    theme_id: uuid::Uuid,
    setting: ComponentSetting,
) -> impl IntoView {
    let (value, set_value) = create_signal(setting.value.clone());
    let (is_updating, set_is_updating) = create_signal(false);
    let setting_id = setting.id;
    let setting_key = setting.key.clone();

    let theme = use_context::<ThemeContext>().expect("ThemeContext not found");

    let handle_save = move |_| {
        let new_value = value.get();
        let key = setting_key.clone();
        let theme = theme.clone();
        set_is_updating.set(true);
        
        spawn_local(async move {
            match api::update_theme_setting(theme_id, setting_id, new_value.clone()).await {
                Ok(_) => {
                    // Update theme context and apply
                    theme.set(key.clone(), new_value);
                    theme.apply_to_document();
                }
                Err(e) => {
                    log::error!("Failed to update setting: {}", e);
                }
            }
            set_is_updating.set(false);
        });
    };

    let is_color = setting.key.starts_with("color_");
    
    view! {
        <div class="border border-slate-200 dark:border-slate-700 rounded-lg p-4 bg-slate-50 dark:bg-slate-900">
            <label class="block mb-2">
                <span class="text-sm font-semibold text-slate-700 dark:text-slate-300">
                    {setting.key.replace("_", " ").to_uppercase()}
                </span>
                {setting.description.map(|desc| view! {
                    <span class="block text-xs text-slate-500 dark:text-slate-400 mt-1">{desc}</span>
                })}
            </label>
            
            <div class="flex gap-2 items-center">
                {if is_color {
                    view! {
                        <input
                            type="color"
                            prop:value=value
                            on:input=move |ev| set_value.set(event_target_value(&ev))
                            class="h-10 w-20 border-2 border-slate-200 dark:border-slate-600 rounded cursor-pointer bg-white dark:bg-slate-700"
                        />
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }}
                
                <input
                    type="text"
                    prop:value=value
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    class="flex-1 px-3 py-2 border-2 border-slate-200 dark:border-slate-600 rounded-lg text-sm bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
                
                <button
                    on:click=handle_save
                    disabled=is_updating.get()
                    style="background-color: var(--color-primary);"
                    class="px-4 py-2 text-white rounded-lg text-sm font-semibold hover:opacity-90 transition-all disabled:opacity-60 disabled:cursor-not-allowed"
                >
                    {move || if is_updating.get() { "Saving..." } else { "Save" }}
                </button>
            </div>
            
            {if is_color {
                view! {
                    <div class="mt-2 flex items-center gap-2">
                        <div
                            class="w-full h-8 rounded border border-slate-200 dark:border-slate-600"
                            style=move || format!("background-color: {}", value.get())
                        />
                    </div>
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
        </div>
    }
}
