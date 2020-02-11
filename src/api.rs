use mongodb::Database;
use serde::Serialize;
use warp::{any, path, reply, Filter, Rejection, Reply};

#[derive(Serialize)]
pub struct ApiVersion {
    pub version: &'static str,
}

pub fn api(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = path("api");

    let api_version = any().and(path::end()).map(|| {
        reply::json(&ApiVersion {
            version: env!("CARGO_PKG_VERSION"),
        })
    });

    any().and(api.and(api_version))
}
