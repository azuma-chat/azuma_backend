use warp::{filters::path, Filter, Rejection};

pub fn get() -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::get().and(path::end())
}
