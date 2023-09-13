//! # Web API
//!
//! ## Routes
//!
//! | METHOD | ROUTE         | WORK                       | RETURN                                      |
//! |--------|---------------|----------------------------|---------------------------------------------|
//! | GET    | `/test`       |                            | Only return -> "OK"                         |
//! | GET    | `/`           | Get Food in fridge from DB | Return a page that list Food in the fridge  |
//! | POST   | `/`           | Add Food in the DB.        | Redirect to `/`                             |
//! | GET    | `/delete/:id` | Remove Food in the DB.     | Redirect to `/`                             |
use axum::routing::{get, post};
use axum::{Extension, Router};
use sqlx::PgPool;

pub mod error;
pub mod food;
pub mod handler;
pub mod template;

/// Defines and returns the router of the app.
pub fn app(db: PgPool) -> Router {
    Router::new()
        .route("/test", get(|| async { "OK" }))
        .route("/", get(handler::list_food))
        .route("/", post(handler::add_food))
        .route("/delete/:id", get(handler::delete_food))
        .layer(Extension(db))
}
