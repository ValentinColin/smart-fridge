/// # http management
use axum::routing::{get, post};
use axum::{Extension, Router};
use sqlx::PgPool;

pub mod error;
pub mod food;
pub mod handler;
pub mod template;

/// Defines and returns the application router.
pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/test", get(|| async { "OK" }))
        .route("/", get(handler::list_food))
        .route("/", post(handler::add_food))
        .route("/delete/:id", get(handler::delete_food))
        .layer(Extension(db))
}
