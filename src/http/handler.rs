//! Define function that will react to each request according the route.
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Form, Json};
use chrono::NaiveDate;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::{PgPool, Row};

use crate::http::food::Food;

/// Struct that get the data from the POST.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormAddFood {
    name: String,
    expiration_date: NaiveDate,
}

/// Get food in the database.
pub async fn get_food(
    db: Extension<PgPool>,
    Path(food_id): Path<Uuid>,
) -> Result<Json<Food>, StatusCode> {
    debug!("get food: {food_id}");

    let response = sqlx::query_as(
        // language=PostgreSQL
        r#"
            SELECT id, name, expiration_date FROM food
            WHERE id = $1::UUID
        "#,
    )
    .bind(food_id.to_string())
    .fetch_one(&*db)
    .await;

    match response {
        Ok(food) => Ok(Json(food)),
        Err(e) => {
            error!("Query error: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// List food in the database.
pub async fn list_food(db: Extension<PgPool>) -> Result<Json<Vec<Food>>, StatusCode> {
    debug!("Food listed");

    let response = sqlx::query(
        // language=PostgreSQL
        r#"
            SELECT id, name, expiration_date FROM food ORDER BY expiration_date
        "#,
    )
    .fetch_all(&*db)
    .await;

    match response {
        Ok(rows) => {
            // Avoid unnecessary system calls
            let mut fridge = Vec::with_capacity(rows.len());

            rows.iter().for_each(|r| {
                fridge.push(Food::new(
                    r.get::<Uuid, _>("id"),
                    r.get::<String, _>("name"),
                    r.try_get::<NaiveDate, _>("expiration_date")
                        .unwrap_or(NaiveDate::MAX),
                ));
            });

            Ok(Json(fridge))
        }
        Err(e) => {
            error!("Query error: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Add food in the database.
pub async fn add_food(db: Extension<PgPool>, Form(form): Form<FormAddFood>) -> StatusCode {
    debug!("Food added: {}", form.name.as_str());

    let food_name = form.name.as_str();
    let expiration_date = form.expiration_date;
    let response = sqlx::query(
        // language=PostgreSQL
        r#"
                INSERT INTO food (name, expiration_date)
                VALUES ($1, $2)
                RETURNING id, name, expiration_date
            "#,
    )
    .bind(food_name)
    .bind(expiration_date)
    .fetch_one(&*db)
    .await;

    match response {
        Ok(_) => StatusCode::NO_CONTENT, // TODO: return the location of the new entry
        Err(e) => {
            error!("Query error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Delete food from the database.
pub async fn delete_food(db: Extension<PgPool>, Path(food_id): Path<Uuid>) -> StatusCode {
    debug!("Food deleted: {}", food_id);

    let response = sqlx::query(
        // language=PostgreSQL
        r#"
            DELETE FROM "food"
            WHERE id = $1::UUID
        "#,
    )
    .bind(food_id)
    .execute(&*db)
    .await;

    match response {
        Ok(_) => StatusCode::NO_CONTENT, // or StatusCode::OK if there is an entity describing the status
        Err(e) => {
            error!("Query error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
