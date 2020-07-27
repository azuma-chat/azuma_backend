use crate::{
    db::with_pool,
    rejection,
    user::{login, registration, session, user},
};
use serde::Serialize;
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{any, body, get, path, post, reply, Filter, Reply};

#[derive(Serialize)]
pub struct ApiVersion {
    pub version: &'static str,
}

pub async fn api(pool: PgPool) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let api_version = path::end().map(|| {
        reply::json(&ApiVersion {
            version: env!("CARGO_PKG_VERSION"),
        })
    });

    // User Routes
    let login_route = post()
        .and(path!("login"))
        .and(body::json())
        .and(with_pool(pool.clone()))
        .and_then(login::login_handler);

    let registration_route = post()
        .and(path!("register"))
        .and(body::json())
        .and(with_pool(pool.clone()))
        .and_then(registration::registration_handler);

    let me_route = get()
        .and(path!("me"))
        .and(session::with_session(pool.clone()))
        .and(with_pool(pool))
        .and_then(user::me_handler);

    let user_routes = login_route.or(registration_route).or(me_route);

    any()
        .and(api_version.or(user_routes))
        .recover(rejection::handle_rejection)
}
