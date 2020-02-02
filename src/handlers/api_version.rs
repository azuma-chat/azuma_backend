use crate::models::api_version::ApiVersion;
use warp::{reply, Rejection, Reply};

pub async fn get() -> Result<impl Reply, Rejection> {
    Ok(reply::json(&ApiVersion {
        version: env!("CARGO_PKG_VERSION"),
    }))
}
