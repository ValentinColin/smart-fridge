//! Module that represent [`Food`] in a fridge.
use chrono::NaiveDate;
use sqlx::types::Uuid;

/// Food struct that represent a row in the database.
#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize, impl_new::New)]
pub struct Food {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) expiration_date: NaiveDate,
}
