use leptos::*;
use platform_core::ComponentSetting;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

const THEME_STORAGE_KEY: &str = "atom_theme_settings";

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub settings: RwSignal<HashMap<String, String>>,
}

impl ThemeContext {
    pub fn new() -> Self {
        let initial_settings = Self::load_from_local_storage();
        
        Self {
            settings: create_rw_signal(initial_settings),
        }
    }

    fn load_from_local_storage() -> HashMap<String, String> {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json_str)) = storage.get_item(THEME_STORAGE_KEY) {
                    if let Ok(settings) = serde_json::from_str::<HashMap<String, String>>(&json_str) {
                        return settings;
                    }
                }
            }
        }
        HashMap::new()
    }

    fn save_to_local_storage(&self) {
        let settings = self.settings.get();
        
        if let Ok(json_str) = serde_json::to_string(&settings) {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(THEME_STORAGE_KEY, &json_str);
                }
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.settings.get().get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        self.settings.update(|settings| {
            settings.insert(key, value);
        });
        self.save_to_local_storage();
    }

    pub fn load_from_api(&self, settings_list: Vec<ComponentSetting>) {
        self.settings.update(|settings| {
            for setting in settings_list {
                settings.insert(setting.key, setting.value);
            }
        });
        self.save_to_local_storage();
    }

    pub fn get_theme_mode(&self) -> String {
        self.settings
            .get()
            .get("theme_mode")
            .cloned()
            .unwrap_or_else(|| "light".to_string())
    }

    pub fn apply_to_document(&self) {
        let settings = self.settings.get();
        
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    // Apply dark/light mode class
                    if let Some(theme_mode) = settings.get("theme_mode") {
                        if theme_mode == "dark" {
                            let _ = root.set_class_name("dark");
                        } else {
                            let _ = root.set_class_name("");
                        }
                    }
                    
                    if let Some(html_element) = root.dyn_ref::<web_sys::HtmlElement>() {
                        let style = html_element.style();
                    
                    // Apply color settings
                    if let Some(primary) = settings.get("color_primary") {
                        let _ = style.set_property("--color-primary", primary);
                    }
                    if let Some(secondary) = settings.get("color_secondary") {
                        let _ = style.set_property("--color-secondary", secondary);
                    }
                    if let Some(tertiary) = settings.get("color_tertiary") {
                        let _ = style.set_property("--color-tertiary", tertiary);
                    }
                    
                    // Apply spacing settings
                    for key in ["spacing_xs", "spacing_sm", "spacing_md", "spacing_lg", "spacing_xl"] {
                        if let Some(value) = settings.get(key) {
                            let css_var = format!("--{}", key.replace("_", "-"));
                            let _ = style.set_property(&css_var, value);
                        }
                    }
                    
                        // Apply radius settings
                        for key in ["radius_sm", "radius_md", "radius_lg", "radius_xl"] {
                            if let Some(value) = settings.get(key) {
                                let css_var = format!("--{}", key.replace("_", "-"));
                                let _ = style.set_property(&css_var, value);
                            }
                        }
                    }
                }
            }
        }
    }
}
