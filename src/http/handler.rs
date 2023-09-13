//! Define function that will react to each request according the route.

use axum::extract::Path;
use axum::response::Redirect;
use axum::{Extension, Form};
use chrono::NaiveDate;
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

use crate::http::food::Food;
use crate::http::template::FridgeTemplate;

/// Struct that get the data from the POST.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormAddFood {
    name: String,
    expiration_date: NaiveDate,
}

/// List food in the fridge.
///
/// TODO: Use the cache to avoid spam requests to the database (after add/delete).
pub async fn list_food(db: Extension<PgPool>) -> FridgeTemplate {
    let rows = sqlx::query(
        // language=PostgreSQL
        r#"
            SELECT id, name, expiration_date FROM food ORDER BY expiration_date
        "#,
    )
    .fetch_all(&*db)
    .await
    .unwrap();

    debug!("Food listed");

    // TODO: manage html error, code...

    // Avoid unnecessary system calls
    let mut fridge = Vec::with_capacity(rows.len());

    rows.iter().for_each(|r| {
        fridge.push(Food::new(
            r.get::<i64, _>("id"),
            r.get::<String, _>("name"),
            r.try_get::<NaiveDate, _>("expiration_date")
                .unwrap_or(NaiveDate::MAX),
        ));
    });

    FridgeTemplate::new(fridge)
}

/// Add food in the database then redirect the client to "/".
pub async fn add_food(db: Extension<PgPool>, Form(form): Form<FormAddFood>) -> Redirect {
    let food_name = form.name.as_str();
    let expiration_date = form.expiration_date;
    let _row: (i64,) = sqlx::query_as(
        // language=PostgreSQL
        r#"
                INSERT INTO food (name, expiration_date)
                VALUES ($1, $2)
                RETURNING id
            "#,
    )
    .bind(food_name)
    .bind(expiration_date)
    .fetch_one(&*db)
    .await
    .unwrap();

    debug!("Food added: {}", food_name);

    // TODO: manage html error, code...

    Redirect::to("/")
}

/// Delete food from the database then redirect the client to "/".
pub async fn delete_food(db: Extension<PgPool>, Path(id): Path<i64>) -> Redirect {
    sqlx::query(
        // language=PostgreSQL
        r#"
            DELETE FROM "food"
            WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&*db)
    .await
    .unwrap();

    debug!("Food deleted: {}", id);

    // TODO: manage html error, code...

    Redirect::to("/")
}
