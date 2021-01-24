use std::env;

use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::mysql::MySqlConnectOptions;

pub type SqlPool = sqlx::MySqlPool;

#[derive(Clone)]
pub struct State {
    pub sql: SqlPool,
}

impl State {
    pub fn new(sql: SqlPool) -> Self {
        Self { sql }
    }
}

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn new_prod() -> Self {
        let username = env::var("USERNAME").expect("USERNAME is not set in .env file");
        let password = env::var("PASSWORD").expect("PASSWORD is not set in .env file");
        let port = env::var("DB_PORT")
            .expect("DB_PORT is not set in .env file")
            .parse::<u16>()
            .expect("Could not convert DB_PORT to a u16 nubmer");
        let host = env::var("DB_HOST").expect("DB_HOST is not set in .env file");
        let database_name =
            env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
        Self {
            username,
            password,
            port,
            host,
            database_name,
        }
    }

    pub fn new_test() -> Self {
        let mut prod_db = Self::new_prod();
        // Perform override
        let database_name =
            env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME is not set in .env file");
        prod_db.database_name = database_name;
        prod_db
    }

    pub fn without_db(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.database_name)
    }
}
