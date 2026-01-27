use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ComponentSetting;

/// Theme
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Theme {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Theme with its settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeWithSettings {
    pub theme: Theme,
    pub settings: Vec<ComponentSetting>,
}

/// Request body for creating a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeCreate {
    pub name: String,
}

/// Request body for updating a theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeUpdate {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}
