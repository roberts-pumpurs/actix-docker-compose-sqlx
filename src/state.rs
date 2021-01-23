pub type SqlPool = sqlx::MySqlPool;
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;

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