use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Component setting item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ComponentSetting {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub category: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for creating a component setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSettingCreate {
    pub key: String,
    pub value: String,
    pub category: String,
    pub description: Option<String>,
}

/// Request body for updating a component setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSettingUpdate {
    pub value: Option<String>,
    pub description: Option<String>,
}

/// Grouped settings by category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsGroup {
    pub category: String,
    pub settings: Vec<ComponentSetting>,
}

// Re-export for backwards compatibility (used by Theme type)
pub type ThemeSetting = ComponentSetting;
