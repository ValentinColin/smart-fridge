//! # Routes
//!
//! | METHOD | ROUTE               | DESCRIPTION                                 | RETURN                   |
//! |--------|---------------------|---------------------------------------------|--------------------------|
//! | GET    | /api/v2/healthcheck | Used to check the health of the http server | (200, body: "OK")        |
//! | GET    | /api/v2/food        | Get all row/food from the database          | (200, body: JSON) or 500 |
//! | POST   | /api/v2/food        | Add food in the database                    | 204 or 500               |
//! | GET    | /api/v2/food/:uuid  | Get one row/food from the database          | (200, body: JSON) or 500 |
//! | DELETE | /api/v2/food/:uuid  | Delete on row/food in the database          | 204 or 500               |
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Router};
use sqlx::PgPool;

use crate::API_PREFIX;

pub mod error;
pub mod food;
pub mod handler;

/// Defines and returns the router of the app.
pub fn app(db: PgPool) -> Router {
    Router::new()
        // common route
        .route(
            format!("{API_PREFIX}/healthcheck").as_str(),
            get(|| async { (StatusCode::OK, "OK") }),
        )
        // cleanup param route
        //.route("/api/v2/cleanup/period", put(|| async { StatusCode::NOT_IMPLEMENTED }))  // TODO
        //.route("/api/v2/cleanup/expiration", put(|| async { StatusCode::NOT_IMPLEMENTED }))  // TODO
        // app route
        .route(
            format!("{API_PREFIX}/food").as_str(),
            get(handler::list_food).post(handler::add_food),
        )
        .route(
            format!("{API_PREFIX}/food/:id").as_str(),
            get(handler::get_food)
                //.put(handler::update_food),  // TODO
                .delete(handler::delete_food),
        )
        // Middleware
        .layer(Extension(db)) // Share the db across handlers
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
            .oneshot(
                Request::builder()
                    .uri("/api/v2/healthcheck")
                    .body(Body::empty())
                    .unwrap(),
            )
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
