use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

use crate::http::food::Food;

/// Struct used to complete the html template.
#[derive(Template)]
#[template(path = "fridge.html")]
pub struct FridgeTemplate {
    fridge: Vec<Food>,
}

impl FridgeTemplate {
    /// Create a new instance of [`FridgeTemplate`].
    pub fn new(fridge: Vec<Food>) -> Self {
        FridgeTemplate { fridge }
    }
}

/// Transform a custom struct into a [`axum_core::response`].
impl IntoResponse for FridgeTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
