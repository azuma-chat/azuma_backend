use mongodb::{Client, Database};
use std::{convert::Infallible, env};
use warp::{any, Filter};

pub async fn create_db() -> Database {
    let db_client = Client::with_uri_str(
        &env::var("AZUMA_MONGODB").expect("Environment variable AZUMA_MONGODB not found"),
    )
    .await
    .expect("Error creating MongoDB client");

    db_client
        .database(&env::var("AZUMA_DBNAME").expect("Environment variable AZUMA_DBNAME not found"))
}

pub fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = Infallible> + Clone {
    any().map(move || db.clone())
}

/*static mut DATABASE: Option<Database> = None;

pub async fn db() -> Database {
    /*unsafe {
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
    }*/
    let db_client = Client::with_uri_str(
        &env::var("AZUMA_MONGODB").expect("Environment variable AZUMA_MONGODB not found"),
    )
    .await
    .expect("Error creating MongoDB client");

    db_client
        .database(&env::var("AZUMA_DBNAME").expect("Environment variable AZUMA_DBNAME not found"))
}
*/
