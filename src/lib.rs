//! # API
//!
//! | METHOD | ROUTE               | DESCRIPTION                                 | RETURN                   |
//! |--------|---------------------|---------------------------------------------|--------------------------|
//! | GET    | /api/v2/healthcheck | Used to check the health of the http server | (200, body: "OK")        |
//! | GET    | /api/v2/food        | Get all row/food from the database          | (200, body: JSON) or 500 |
//! | POST   | /api/v2/food        | Add food in the database                    | 204 or 500               |
//! | GET    | /api/v2/food/:uuid  | Get one row/food from the database          | (200, body: JSON) or 500 |
//! | DELETE | /api/v2/food/:uuid  | Delete on row/food in the database          | 204 or 500               |

/// Module that contains some database functions.
pub mod db;

/// Module that manage all the web API.
pub mod http;

pub const API_PREFIX: &str = "/api/v2";
