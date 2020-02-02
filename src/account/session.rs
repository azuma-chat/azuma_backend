use mongodb::Database;
use warp::{cookie, reject, Filter, Rejection};

pub struct Session {
    pub userid: i64,
}

pub fn session_middleware(
    db: Database,
) -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    cookie::optional("azuma_session").and_then(|session_cookie| {
        async move {
            println!("{:?}", session_cookie);
            Err(reject::not_found())
        }
    })
}
