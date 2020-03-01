use crate::{model::session::Session, rejection::AzumaRejection};
use chrono::{Duration, Utc};
use warp::{cookie, http::Response, reject, Filter, Rejection, Reply};

pub fn with_session() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    cookie::optional("azuma_session").and_then(|session_cookie| async move {
        match session_cookie {
            Some(session_cookie) => match Session::get(session_cookie) {
                Ok(session) => match session.validate() {
                    Ok(_) => Ok(session),
                    Err(why) => Err(reject::custom(why)),
                },
                Err(why) => Err(reject::custom(why)),
            },
            None => Err(reject::custom(AzumaRejection::Unauthorized)),
        }
    })
}

pub async fn update_session(forwarded: (impl Reply, Session)) -> Result<impl Reply, Rejection> {
    let mut response = Response::builder();

    let session = forwarded.1;
    if session.expiration < (Utc::now() + Duration::days(7)) {
        match Session::new(session.userid) {
            Ok(new_session) => {
                response = response.header(
                    "Set-Cookie",
                    format!(
                        "azuma_session={}; Max-Age=2592000; Path=/",
                        new_session.token
                    ),
                );
            }
            Err(why) => return Err(reject::custom(why)),
        }
    }

    let original_response = forwarded.0.into_response();
    Ok(response
        .status(original_response.status())
        .body(original_response.into_body()))
}
