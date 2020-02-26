use crate::{rejection, user::session};
use serde::Serialize;
use std::convert::Infallible;
use warp::{any, path, reply, Filter, Reply};

#[derive(Serialize)]
pub struct ApiVersion {
    pub version: &'static str,
}

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    let api = path("api");

    let api_version = any().and(path::end()).map(|| {
        reply::json(&ApiVersion {
            version: env!("CARGO_PKG_VERSION"),
        })
    });

    let protected = any()
        .and(path!("protected"))
        .and(session::with_session())
        .map(|session| {
            (
                reply::json(&ApiVersion {
                    version: env!("CARGO_PKG_VERSION"),
                }),
                session,
            )
        })
        .and_then(session::update_session);

    any()
        .and(api.and(api_version.or(protected)))
        .recover(rejection::handle_rejection)
}
