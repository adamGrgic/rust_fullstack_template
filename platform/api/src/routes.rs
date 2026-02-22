use axum::{
    routing::{get, post},
    Router,
};

use crate::{db::Database, handlers::{todos, component_settings, themes}};

pub fn create_router(db: Database) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/todos", get(todos::list_todos).post(todos::create_todo))
        .route(
            "/api/todos/:id",
            get(todos::get_todo)
                .put(todos::update_todo)
                .delete(todos::delete_todo),
        )
        .route("/api/settings", get(component_settings::list_settings))
        .route(
            "/api/settings/:id",
            get(component_settings::get_setting)
                .put(component_settings::update_setting),
        )
        .route("/api/themes", get(themes::list_themes).post(themes::create_theme))
        .route("/api/themes/:id", get(themes::get_theme_with_settings).put(themes::update_theme).delete(themes::delete_theme))
        .route("/api/themes/:theme_id/settings/:setting_id", axum::routing::put(themes::update_theme_setting))
        .with_state(db)
}

async fn health_check() -> &'static str {
    "OK"
}

