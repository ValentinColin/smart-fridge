//! Module that represent [`Food`] in a fridge.

/// Food struct that represent a row in the database.
#[derive(Debug, sqlx::FromRow, serde::Deserialize)]
pub struct Food {
    pub(crate) id: i64, // sqlx don't know decode usize
    pub(crate) name: String,
}

impl Food {
    /// Creates a [`Food`] instance.
    pub fn new(id: i64, name: String) -> Self {
        Food { id, name }
    }
}
