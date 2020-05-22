use crate::{model::session::Session, rejection::AzumaRejection};
use chrono::{Duration, Utc};
use warp::{header, http::Response, reject, Filter, Rejection, Reply};

pub async fn with_session() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    header::header("Authorization")
        .and_then(|mut authentication_header: String| async move {
            let authentication_token = authentication_header.split_off(7);
            if authentication_header == "Bearer " {
                match Session::get(authentication_token).await {
                    Ok(session) => Ok(session),
                    Err(why) => Err(reject::custom(why)),
                }
            } else {
                Err(reject::custom(AzumaRejection::Unauthorized))
            }
        })
        .or_else(|_| async move { Err(reject::custom(AzumaRejection::Unauthorized)) })
}

pub async fn update_session(forwarded: (impl Reply, Session)) -> Result<impl Reply, Rejection> {
    let mut response = Response::builder();

    let session = forwarded.1;
    if session.expiration < (Utc::now() + Duration::days(7)) {
        let new_session = Session::new(session.userid).await?;
        response = response.header("Authorization", format!("Bearer {}", new_session.token));
    }

    let original_response = forwarded.0.into_response();
    Ok(response
        .status(original_response.status())
        .body(original_response.into_body()))
}
