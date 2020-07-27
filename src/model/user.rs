use crate::rejection::AzumaRejection;
use chrono::{DateTime, Utc};
use pbkdf2::pbkdf2_simple;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub created: DateTime<Utc>,
    pub name: String,
    pub password: String,
    pub icon: Option<String>,
    pub status: Option<String>,
}

impl User {
    pub async fn new(
        name: String,
        password: String,
        pool: &PgPool,
    ) -> Result<User, AzumaRejection> {
        let user = query("select 1 from users where name = $1")
            .bind(&name)
            .fetch_optional(pool)
            .await?;

        match user {
            None => {
                let hashed_password = pbkdf2_simple(&password, 100000).expect("RNG error");
                let user: User =
                    query_as("insert into users (name, password) values ($1, $2) returning *")
                        .bind(name)
                        .bind(hashed_password)
                        .fetch_one(pool)
                        .await?;
                Ok(user)
            }
            Some(_) => Err(AzumaRejection::AlreadyExists),
        }
    }

    pub async fn get(name: String, pool: &PgPool) -> Result<User, AzumaRejection> {
        let user: User = query_as("select * from users where name = $1")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_id(id: i64, pool: &PgPool) -> Result<User, AzumaRejection> {
        let user: User = query_as("select * from users where id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
}
