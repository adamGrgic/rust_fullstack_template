use gloo_net::http::Request;
use platform_core::{Todo, TodoCreate, TodoUpdate, ComponentSetting, ComponentSettingUpdate, Theme, ThemeWithSettings, ThemeCreate, ThemeUpdate};
use uuid::Uuid;

const API_BASE: &str = "http://localhost:8080/api";

pub async fn fetch_todos() -> Result<Vec<Todo>, String> {
    let response = Request::get(&format!("{}/todos", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch todos: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn create_todo(todo: TodoCreate) -> Result<Todo, String> {
    let response = Request::post(&format!("{}/todos", API_BASE))
        .json(&todo)
        .map_err(|e| format!("Failed to serialize todo: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to create todo: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_todo(id: Uuid, update: TodoUpdate) -> Result<Todo, String> {
    let response = Request::put(&format!("{}/todos/{}", API_BASE, id))
        .json(&update)
        .map_err(|e| format!("Failed to serialize update: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to update todo: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn delete_todo(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{}/todos/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Failed to delete todo: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    Ok(())
}

// Component Settings API
pub async fn fetch_settings() -> Result<Vec<ComponentSetting>, String> {
    let response = Request::get(&format!("{}/settings", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch settings: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_setting(id: Uuid, update: ComponentSettingUpdate) -> Result<ComponentSetting, String> {
    let response = Request::put(&format!("{}/settings/{}", API_BASE, id))
        .json(&update)
        .map_err(|e| format!("Failed to serialize update: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

// Themes API
pub async fn fetch_themes() -> Result<Vec<Theme>, String> {
    let response = Request::get(&format!("{}/themes", API_BASE))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch themes: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn fetch_theme_with_settings(id: Uuid) -> Result<ThemeWithSettings, String> {
    let response = Request::get(&format!("{}/themes/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch theme: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn create_theme(theme: ThemeCreate) -> Result<Theme, String> {
    let response = Request::post(&format!("{}/themes", API_BASE))
        .json(&theme)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to create theme: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_theme(id: Uuid, update: ThemeUpdate) -> Result<Theme, String> {
    let response = Request::put(&format!("{}/themes/{}", API_BASE, id))
        .json(&update)
        .map_err(|e| format!("Failed to serialize update: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to update theme: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn update_theme_setting(theme_id: Uuid, setting_id: Uuid, value: String) -> Result<ComponentSetting, String> {
    let response = Request::put(&format!("{}/themes/{}/settings/{}", API_BASE, theme_id, setting_id))
        .json(&value)
        .map_err(|e| format!("Failed to serialize value: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to update setting: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}
