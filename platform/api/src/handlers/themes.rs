use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use platform_core::{Theme, ThemeWithSettings, ThemeCreate, ThemeUpdate, ComponentSetting};
use uuid::Uuid;

use crate::{db::Database, error::Result};

pub async fn list_themes(State(db): State<Database>) -> Result<Json<Vec<Theme>>> {
    let themes = sqlx::query_as!(
        Theme,
        r#"
        SELECT id, name, is_active, created_at, updated_at
        FROM themes
        ORDER BY name
        "#
    )
    .fetch_all(db.pool())
    .await?;

    Ok(Json(themes))
}

pub async fn get_theme_with_settings(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<ThemeWithSettings>> {
    let theme = sqlx::query_as!(
        Theme,
        r#"
        SELECT id, name, is_active, created_at, updated_at
        FROM themes
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db.pool())
    .await?
    .ok_or(crate::error::AppError::NotFound)?;

    let settings = sqlx::query_as!(
        ComponentSetting,
        r#"
        SELECT id, key, value, category, description, created_at, updated_at
        FROM theme_settings
        WHERE theme_id = $1
        ORDER BY category, key
        "#,
        id
    )
    .fetch_all(db.pool())
    .await?;

    Ok(Json(ThemeWithSettings { theme, settings }))
}

pub async fn create_theme(
    State(db): State<Database>,
    Json(payload): Json<ThemeCreate>,
) -> Result<(StatusCode, Json<Theme>)> {
    let theme = sqlx::query_as!(
        Theme,
        r#"
        INSERT INTO themes (name, is_active)
        VALUES ($1, false)
        RETURNING id, name, is_active, created_at, updated_at
        "#,
        payload.name
    )
    .fetch_one(db.pool())
    .await?;

    // Copy settings from the currently active theme
    let active_theme = sqlx::query!("SELECT id FROM themes WHERE is_active = true LIMIT 1")
        .fetch_one(db.pool())
        .await?;

    sqlx::query!(
        r#"
        INSERT INTO theme_settings (theme_id, key, value, category, description)
        SELECT $1, key, value, category, description
        FROM theme_settings
        WHERE theme_id = $2
        "#,
        theme.id,
        active_theme.id
    )
    .execute(db.pool())
    .await?;

    Ok((StatusCode::CREATED, Json(theme)))
}

pub async fn update_theme(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ThemeUpdate>,
) -> Result<Json<Theme>> {
    let _existing = sqlx::query!("SELECT id FROM themes WHERE id = $1", id)
        .fetch_optional(db.pool())
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    // If setting as active, deactivate all others first
    if payload.is_active == Some(true) {
        sqlx::query!("UPDATE themes SET is_active = false")
            .execute(db.pool())
            .await?;
    }

    let theme = if let Some(name) = payload.name {
        if let Some(is_active) = payload.is_active {
            sqlx::query_as!(
                Theme,
                r#"
                UPDATE themes
                SET name = $1, is_active = $2, updated_at = NOW()
                WHERE id = $3
                RETURNING id, name, is_active, created_at, updated_at
                "#,
                name,
                is_active,
                id
            )
            .fetch_one(db.pool())
            .await?
        } else {
            sqlx::query_as!(
                Theme,
                r#"
                UPDATE themes
                SET name = $1, updated_at = NOW()
                WHERE id = $2
                RETURNING id, name, is_active, created_at, updated_at
                "#,
                name,
                id
            )
            .fetch_one(db.pool())
            .await?
        }
    } else if let Some(is_active) = payload.is_active {
        sqlx::query_as!(
            Theme,
            r#"
            UPDATE themes
            SET is_active = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, name, is_active, created_at, updated_at
            "#,
            is_active,
            id
        )
        .fetch_one(db.pool())
        .await?
    } else {
        return Err(crate::error::AppError::BadRequest(
            "No fields to update".to_string(),
        ));
    };

    Ok(Json(theme))
}

pub async fn delete_theme(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query!("DELETE FROM themes WHERE id = $1 AND is_active = false", id)
        .execute(db.pool())
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::BadRequest(
            "Cannot delete active theme".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Theme settings endpoints
pub async fn update_theme_setting(
    State(db): State<Database>,
    Path((theme_id, setting_id)): Path<(Uuid, Uuid)>,
    Json(value): Json<String>,
) -> Result<Json<ComponentSetting>> {
    let setting = sqlx::query_as!(
        ComponentSetting,
        r#"
        UPDATE theme_settings
        SET value = $1, updated_at = NOW()
        WHERE theme_id = $2 AND id = $3
        RETURNING id, key, value, category, description, created_at, updated_at
        "#,
        value,
        theme_id,
        setting_id
    )
    .fetch_one(db.pool())
    .await?;

    Ok(Json(setting))
}
