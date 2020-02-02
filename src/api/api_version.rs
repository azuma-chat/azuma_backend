use crate::{handlers, routes};
use warp::{Filter, Rejection, Reply};

pub fn get() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    routes::api_version::get().and_then(handlers::api_version::get)
}
