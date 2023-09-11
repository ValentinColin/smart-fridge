//! Module that represent [`Food`] in a fridge.
use chrono::NaiveDate;

/// Food struct that represent a row in the database.
#[derive(Debug, sqlx::FromRow, serde::Deserialize)]
pub struct Food {
    pub(crate) id: i64, // sqlx don't know decode usize
    pub(crate) name: String,
    pub(crate) expiration_date: NaiveDate,
}

impl Food {
    /// Creates a [`Food`] instance.
    pub fn new(id: i64, name: String, expiration_date: NaiveDate) -> Self {
        Food {
            id,
            name,
            expiration_date,
        }
    }
}
