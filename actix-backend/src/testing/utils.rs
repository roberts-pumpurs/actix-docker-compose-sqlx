use actix_web::{web::Data};
use sqlx::{Connection, Executor, MySqlConnection, MySqlPool};
use state::State;
use std::{sync::Arc};

use crate::{
    state::{self, DatabaseSettings},
};

pub struct TestApp {
    pub db_pool: MySqlPool,
    pub state: Data<Arc<State>>,
}

pub async fn spawn_app() -> TestApp {
    let db_pool = configure_test_database().await;

    let state = std::sync::Arc::new(state::State::new(db_pool.clone()));
    let state = state::AppState::new(state);

    TestApp { db_pool, state }
}

async fn configure_test_database() -> MySqlPool {
    // Create database
    let db_settings = DatabaseSettings::new_test();
    let mut connection = MySqlConnection::connect_with(&db_settings.without_db())
        .await
        .expect("Could not connect to the datbase");

    connection
        .execute(&*format!(
            r#"CREATE DATABASE IF NOT EXISTS `{}`;"#,
            db_settings.database_name
        ))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = MySqlPool::connect_with(db_settings.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
