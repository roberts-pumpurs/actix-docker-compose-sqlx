use serde::{Deserialize, Serialize};
use sqlx::Done;
use sqlx::{FromRow};

use crate::state::AppState;

type SqlID = u64;

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
    async fn user_add(&self, form: &Register) -> sqlx::Result<SqlID>;
    async fn user_query(&self, id: SqlID) -> sqlx::Result<User>;
    async fn user_all(&self) -> sqlx::Result<Vec<User>>;
}

#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<SqlID> {
        let id = sqlx::query!(
            r#"
        INSERT INTO users (name, email)
        VALUES (?, ?);
                "#,
            form.name,
            form.email,
        )
        .execute(&self.sql)
        .await?
        .last_insert_id();
        Ok(id)
    }

    async fn user_query(&self, id: SqlID) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email
        FROM users
        where id = ?
                "#,
            id
        )
        .fetch_one(&self.sql)
        .await
    }

    async fn user_all(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email
        FROM users
        ORDER BY id
            "#,
        )
        .fetch_all(&self.sql)
        .await
    }
}
