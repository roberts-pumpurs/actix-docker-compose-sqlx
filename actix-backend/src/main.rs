#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use actix_web;
use anyhow::Result;
use dotenv::dotenv;
use sqlx::MySqlPool;
use state::DatabaseSettings;
use std::env;

mod testing;
mod api;
mod how;
mod routes;
mod startup;
mod state;
mod todo;

#[actix_web::main]
async fn main() -> Result<()> {
    // ---------------------------------- //
    dotenv().ok();
    env_logger::init();

    // ---------------------------------- //
    // Construct the sharable state between workers (Database pool)
    let db_settings = DatabaseSettings::new_prod().with_db();
    let db_pool = MySqlPool::connect_with(db_settings).await?;
    let state = std::sync::Arc::new(state::State::new(db_pool));
    let state = state::AppState::new(state);

    // ---------------------------------- //
    // Run the server
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    startup::run(state, host, port)?.await?;

    Ok(())
}
