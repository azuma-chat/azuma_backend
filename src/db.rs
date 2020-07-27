use sqlx::{
    migrate,
    postgres::{PgPool, PgPoolOptions},
};
use std::{convert::Infallible, env};
use warp::{any, Filter};

pub async fn create_pool() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &env::var("AZUMA_POSTGRESQL").expect("Environment variable AZUMA_POSTGRESQL not found"),
        )
        .await
        .expect("Couldn't connect to PostgreSQL database");
    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");
    pool
}

pub fn with_pool(pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    any().map(move || pool.clone())
}
