//! # Entry point of the app
use anyhow::Context;
use log::{debug, info};
use smart_fridge::http::app;
use sqlx::postgres::PgPoolOptions;

/// # Entry point
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Server Starting...");

    // DATABASE
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&get_db_url()?)
        .await
        .context("Failed to connect to the database")?;
    debug!("✅Connection to the database is successful!");

    // TODO: Faire une migration conditionnelle
    info!("Migrate database...");
    sqlx::migrate!("db/migrations").run(&db).await?;

    // HTTP APP
    let addr = format!(
        "{}:{}",
        dotenvy::var("WEB_APP_HOST").unwrap_or("0.0.0.0".to_string()),
        dotenvy::var("WEB_APP_PORT").unwrap_or("80".to_string())
    );
    info!("Listening on http://{}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .unwrap();

    Ok(())
}

/// Try to get the database URL from env vars.
///
/// Shell variables have priority over file variables.
///
/// # Err
///
/// Return an [`Err`] if no password has been set.
fn get_db_url() -> anyhow::Result<String> {
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
