use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a todo item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "todo_status", rename_all = "lowercase"))]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl TodoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoStatus::Pending => "pending",
            TodoStatus::InProgress => "in_progress",
            TodoStatus::Completed => "completed",
            TodoStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Complete todo item with all fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for creating a new todo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoCreate {
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<TodoStatus>,
}

/// Request body for updating an existing todo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
}

