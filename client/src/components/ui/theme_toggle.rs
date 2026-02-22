use leptos::*;
use crate::theme::ThemeContext;
use crate::api;
use platform_core::ComponentSettingUpdate;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_context::<ThemeContext>().expect("ThemeContext not found");
    
    let theme_for_memo = theme.clone();
    let current_mode = create_memo(move |_| theme_for_memo.get_theme_mode());
    let is_dark = create_memo(move |_| current_mode.get() == "dark");

    let toggle_theme = move |_| {
        let new_mode = if is_dark.get() { "light" } else { "dark" };
        let theme = theme.clone();
        
        spawn_local(async move {
            // Find the theme_mode setting ID
            match api::fetch_settings().await {
                Ok(settings) => {
                    if let Some(setting) = settings.iter().find(|s| s.key == "theme_mode") {
                        let update = ComponentSettingUpdate {
                            value: Some(new_mode.to_string()),
                            description: None,
                        };
                        
                        match api::update_setting(setting.id, update).await {
                            Ok(_) => {
                                theme.set("theme_mode".to_string(), new_mode.to_string());
                                theme.apply_to_document();
                            }
                            Err(e) => {
                                log::error!("Failed to update theme mode: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to fetch settings: {}", e);
                }
            }
        });
    };

    view! {
        <button
            on:click=toggle_theme
            class="px-3 py-2 rounded-lg bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors flex items-center gap-2"
            title=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
        >
            {move || if is_dark.get() {
                view! {
                    <span>"☀️"</span>
                }.into_view()
            } else {
                view! {
                    <span>"🌙"</span>
                }.into_view()
            }}
            <span class="text-sm font-medium">
                {move || if is_dark.get() { "Light" } else { "Dark" }}
            </span>
        </button>
    }
}
