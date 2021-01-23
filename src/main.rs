#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use actix_web::{App, HttpServer, middleware::{Logger}};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::MySqlPool;
use std::env;

mod api;
mod routes;
mod todo;
mod state;
mod how;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // ---------------------------------- //
    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    // ---------------------------------- //
    // Construct the sharable state between workers (Database pool)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::connect(&database_url).await?;
    let state = std::sync::Arc::new(state::State::new(db_pool));
    let state = state::AppState::new(state);
    // let state = actix_web::web::Data::new(state);

    // ---------------------------------- //
    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .data(state.clone()) // pass database pool to application so we can access it inside handlers
            .wrap(Logger::default())
            .configure(routes::routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
