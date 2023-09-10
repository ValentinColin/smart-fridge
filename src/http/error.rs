//! Module that manages any internal errors that may occur.

/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    /// Template engine error
    #[error("an internal server error occurred")]
    Askama(#[from] askama::Error),
}
