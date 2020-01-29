use core::convert::Infallible;
use mongodb::Database;
use warp::{cookie, Filter};

pub fn session_middleware(db: Database) -> impl Filter<Extract = (), Error = Infallible> + Clone {
    cookie::optional("azuma_session")
        .map(|session_cookie| {
            println!("{:?}", session_cookie);
        })
        .untuple_one()
}
