use crate::account::session;
use mongodb::Database;
use warp::{any, get, path, Filter, Rejection, Reply};

pub fn api(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = path("api");

    let api_version = get()
        .and(path::end())
        .and(session::session_middleware(db.clone()))
        .map(|_session| env!("CARGO_PKG_VERSION"));

    any().and(api.and(api_version))
}
