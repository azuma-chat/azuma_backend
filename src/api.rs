use warp::{path, Filter, Rejection, Reply};

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api_version = path("api")
        .and(path::end())
        .map(|| env!("CARGO_PKG_VERSION"));

    warp::any().and(api_version)
}
