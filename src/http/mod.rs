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
use sqlx::{Pool, Postgres};

pub mod error;
pub mod food;
pub mod handler;
pub mod template;

/// Defines and returns the router of the app.
pub fn app(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/test", get(|| async { "OK" }))
        .route("/", get(handler::list_food))
        .route("/", post(handler::add_food))
        .route("/delete/:id", get(handler::delete_food))
        .layer(Extension(db))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::get_db_url;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use sqlx::postgres::PgPoolOptions;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn axum_healthcheck() {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&get_db_url().unwrap())
            .await
            .unwrap();
        let app = app(db);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn axum_not_found() {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&get_db_url().unwrap())
            .await
            .unwrap();
        let app = app(db);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(body.is_empty());
    }
}
