use mongodb::{Client, Database};
use std::env;

static mut DATABASE: Option<Database> = None;

pub async fn db() -> Database {
    unsafe {
        if let Some(db) = &DATABASE {
            db.clone()
        } else {
            let db_client = Client::with_uri_str(
                &env::var("AZUMA_MONGODB").expect("Environment variable AZUMA_MONGODB not found"),
            )
            .await
            .expect("Error creating MongoDB client");

            let db = db_client.database(
                &env::var("AZUMA_DBNAME").expect("Environment variable AZUMA_DBNAME not found"),
            );

            DATABASE = Some(db.clone());
            db
        }
    }
}
