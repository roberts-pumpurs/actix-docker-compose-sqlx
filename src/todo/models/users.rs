use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::Done;
use sqlx::{FromRow, Row};

use crate::state::AppState;

type SqlID = i32;

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: SqlID,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub email: String,
}

#[async_trait]
pub trait IUser {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64>;
    async fn user_query(&self, name: &str) -> sqlx::Result<User>;
}

#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        sqlx::query!(
            r#"
        INSERT INTO users (name, email)
        VALUES (?, ?)
                "#,
            form.name,
            form.email,
        )
        .execute(&self.sql)
        .await
        .map(|d| d.rows_affected())
    }

    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email
        FROM users
        where name = ?
                "#,
            name
        )
        .fetch_one(&self.sql)
        .await
    }
}
