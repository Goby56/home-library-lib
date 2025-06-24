pub mod database;
pub mod routes;
pub mod types;
pub mod auth;

use sqlx::{Pool, Sqlite};

pub struct AppState {
    pub db: Pool<Sqlite>,
}
