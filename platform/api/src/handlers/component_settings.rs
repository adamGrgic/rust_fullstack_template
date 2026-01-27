use axum::{
    extract::{Path, State},
    Json,
};
use platform_core::{ComponentSetting, ComponentSettingUpdate};
use uuid::Uuid;

use crate::{db::Database, error::Result};

pub async fn list_settings(State(db): State<Database>) -> Result<Json<Vec<ComponentSetting>>> {
    let settings = sqlx::query_as!(
        ComponentSetting,
        r#"
        SELECT 
            id,
            key,
            value,
            category,
            description,
            created_at,
            updated_at
        FROM component_settings
        ORDER BY category, key
        "#
    )
    .fetch_all(db.pool())
    .await?;

    Ok(Json(settings))
}

pub async fn get_setting(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<ComponentSetting>> {
    let setting = sqlx::query_as!(
        ComponentSetting,
        r#"
        SELECT 
            id,
            key,
            value,
            category,
            description,
            created_at,
            updated_at
        FROM component_settings
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db.pool())
    .await?
    .ok_or(crate::error::AppError::NotFound)?;

    Ok(Json(setting))
}

pub async fn update_setting(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ComponentSettingUpdate>,
) -> Result<Json<ComponentSetting>> {
    let _existing = sqlx::query!("SELECT id FROM component_settings WHERE id = $1", id)
        .fetch_optional(db.pool())
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    // Simple update focusing on value changes
    if let Some(value) = payload.value {
        let setting = sqlx::query_as!(
            ComponentSetting,
            r#"
            UPDATE component_settings 
            SET value = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, key, value, category, description, created_at, updated_at
            "#,
            value,
            id
        )
        .fetch_one(db.pool())
        .await?;

        Ok(Json(setting))
    } else if let Some(description) = payload.description {
        let setting = sqlx::query_as!(
            ComponentSetting,
            r#"
            UPDATE component_settings 
            SET description = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, key, value, category, description, created_at, updated_at
            "#,
            description,
            id
        )
        .fetch_one(db.pool())
        .await?;

        Ok(Json(setting))
    } else {
        // No changes, just return existing
        get_setting(State(db), Path(id)).await
    }
}
