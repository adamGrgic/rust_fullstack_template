use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use platform_core::{Todo, TodoCreate, TodoStatus, TodoUpdate};
use uuid::Uuid;

use crate::{db::Database, error::Result};

pub async fn list_todos(State(db): State<Database>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT 
            id,
            title,
            description,
            status as "status: TodoStatus",
            created_at,
            updated_at
        FROM todos
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db.pool())
    .await?;

    Ok(Json(todos))
}

pub async fn get_todo(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT 
            id,
            title,
            description,
            status as "status: TodoStatus",
            created_at,
            updated_at
        FROM todos
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db.pool())
    .await?
    .ok_or(crate::error::AppError::NotFound)?;

    Ok(Json(todo))
}

pub async fn create_todo(
    State(db): State<Database>,
    Json(payload): Json<TodoCreate>,
) -> Result<(StatusCode, Json<Todo>)> {
    let status = payload.status.unwrap_or(TodoStatus::Pending);

    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, description, status)
        VALUES ($1, $2, $3)
        RETURNING 
            id,
            title,
            description,
            status as "status: TodoStatus",
            created_at,
            updated_at
        "#,
        payload.title,
        payload.description,
        status as TodoStatus
    )
    .fetch_one(db.pool())
    .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
    Json(payload): Json<TodoUpdate>,
) -> Result<Json<Todo>> {
    // First check if the todo exists
    let _existing = sqlx::query!("SELECT id FROM todos WHERE id = $1", id)
        .fetch_optional(db.pool())
        .await?
        .ok_or(crate::error::AppError::NotFound)?;

    // Build dynamic update query
    let mut query = String::from("UPDATE todos SET updated_at = NOW()");
    let mut params: Vec<String> = vec![];
    let mut param_count = 1;

    if let Some(title) = &payload.title {
        query.push_str(&format!(", title = ${}", param_count));
        params.push(title.clone());
        param_count += 1;
    }

    if payload.description.is_some() {
        query.push_str(&format!(", description = ${}", param_count));
        params.push(payload.description.clone().unwrap_or_default());
        param_count += 1;
    }

    if let Some(status) = &payload.status {
        query.push_str(&format!(", status = ${}", param_count));
        params.push(status.as_str().to_string());
        param_count += 1;
    }

    query.push_str(&format!(" WHERE id = ${}", param_count));

    // Execute update
    let mut sqlx_query = sqlx::query(&query);
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }
    sqlx_query = sqlx_query.bind(id);
    sqlx_query.execute(db.pool()).await?;

    // Fetch and return updated todo
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT 
            id,
            title,
            description,
            status as "status: TodoStatus",
            created_at,
            updated_at
        FROM todos
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db.pool())
    .await?;

    Ok(Json(todo))
}

pub async fn delete_todo(
    State(db): State<Database>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(db.pool())
        .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

