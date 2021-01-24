use actix_web::{App, HttpServer, dev::Server, middleware::Logger};
use log::info;

use crate::routes;
use crate::how::Error;
use crate::state;


pub fn run(
    state: state::AppState,
    host: String,
    port: String,
) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .data(state.clone()) // pass database pool to application so we can access it inside handlers
            .wrap(Logger::default())
            .configure(routes::routes)
    })
    .bind(format!("{}:{}", host, port))?;

    info!("Starting server");
    let server = server.run();

    Ok(server)
}
