use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use time::{ext::NumericalDuration, Duration};

use smart_fridge::db::{cleanup, get_db_url};

#[derive(impl_new::New)]
struct CleanupParam {
    food_expiration_period: Duration,
    check_food_period: Duration,
}

#[tokio::main]
#[allow(unreachable_code)]
async fn main() -> anyhow::Result<()> {
    let db = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&get_db_url()?)
            .await
            .context("Failed to connect to the database")?,
    );

    let cleanup_param = CleanupParam::new(3.days(), 1.hours());

    // TODO: Paramétrage statique ou dynamique ? API http pour modifié les paramètres ?
    // TODO: Ajouter une condition de fin de boucle ?
    loop {
        let _nb_deleted = cleanup(db.clone(), cleanup_param.food_expiration_period).await;
        // TODO: Notifié lorsque des aliments sont supprimmer car périmé depuis trop longtemps

        tokio::time::sleep(std::time::Duration::from_secs_f32(
            cleanup_param.check_food_period.as_seconds_f32(),
        ))
        .await;
    }

    Ok(())
}
