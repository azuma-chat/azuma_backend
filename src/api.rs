pub mod api_version;

use mongodb::Database;
use warp::{any, path, Filter, Rejection, Reply};

pub fn api(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api = path("api");

    any().and(api.and(api_version::get()))
}
