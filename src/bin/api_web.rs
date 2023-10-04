//! # Entry point of the app
use anyhow::Context;
use log::{debug, info};
use smart_fridge::db::get_db_url;
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
