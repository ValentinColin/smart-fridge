use anyhow::Context;
use log::debug;
use sqlx::PgPool;
use std::sync::Arc;
use time::Duration;

/// Try to get the database URL from env vars.
///
/// Shell variables have priority over file variables.
///
/// # Err
///
/// Return an [`Err`] if no password has been set.
pub fn get_db_url() -> anyhow::Result<String> {
    const DB_ADDR: &str = "DATABASE_ADDR";
    const DB_NAME: &str = "DATABASE_NAME";
    const DB_USERNAME: &str = "DATABASE_USERNAME";
    const DB_PASSWORD: &str = "DATABASE_PASSWORD";

    let db_addr = if std::env::var(DB_ADDR).is_ok() {
        std::env::var(DB_ADDR).unwrap()
    } else if dotenvy::var(DB_ADDR).is_ok() {
        dotenvy::var(DB_ADDR).unwrap()
    } else {
        "db".to_string()
    };

    let db_name = if std::env::var(DB_NAME).is_ok() {
        std::env::var(DB_NAME).unwrap()
    } else if dotenvy::var(DB_NAME).is_ok() {
        dotenvy::var(DB_NAME).unwrap()
    } else {
        "postgres".to_string()
    };

    let db_username = if std::env::var(DB_USERNAME).is_ok() {
        std::env::var(DB_USERNAME).unwrap()
    } else if dotenvy::var(DB_NAME).is_ok() {
        dotenvy::var(DB_NAME).unwrap()
    } else {
        "postgres".to_string()
    };

    let db_password = if std::env::var(DB_PASSWORD).is_ok() {
        std::env::var(DB_PASSWORD).unwrap()
    } else {
        dotenvy::var(DB_PASSWORD).context(format!("{DB_PASSWORD} must be set"))?
    };

    debug!(
        "Connect to the database: postgres://{}:******@{}/{}",
        db_username.clone(),
        db_addr.clone(),
        db_name.clone()
    );
    Ok(format!(
        "postgres://{db_username}:{db_password}@{db_addr}/{db_name}"
    ))
}

/// Cleaning the fridge for food that has been out of date for a certain number of days.
pub async fn cleanup(db: Arc<PgPool>, food_expiration_period: Duration) -> i64 {
    let nb_deleted: (i64,) = sqlx::query_as(
        // language=PostgreSQL
        format!(
            r#"
                WITH deleted_rows AS (
                    DELETE FROM food
                    WHERE expiration_date <= current_date - interval '{} days'
                    RETURNING *
                )
                SELECT count(*) as deleted_count FROM deleted_rows
            "#,
            food_expiration_period.whole_days()
        )
        .as_str(),
    )
    .fetch_one(&*db)
    .await
    .unwrap();

    nb_deleted.0
}
