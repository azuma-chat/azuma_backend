use crate::{
    model::session::{Session, SessionLocation},
    rejection::AzumaRejection,
};
use chrono::{Duration, Utc};
use warp::{cookie, header, http::Response, reject, Filter, Rejection, Reply};

pub fn with_session() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    cookie::cookie("azuma_session")
        .and_then(|session_token| async move {
            match Session::get(session_token) {
                Ok(session) => Ok(session.set_location(SessionLocation::Cookie)),
                Err(why) => Err(reject::custom(why)),
            }
        })
        .or(
            header::header("Azuma-Session").and_then(|session_token| async move {
                match Session::get(session_token) {
                    Ok(session) => Ok(session.set_location(SessionLocation::Header)),
                    Err(why) => Err(reject::custom(why)),
                }
            }),
        )
        .unify()
        .or_else(|_| async move { Err(reject::custom(AzumaRejection::Unauthorized)) })
}

pub async fn update_session(forwarded: (impl Reply, Session)) -> Result<impl Reply, Rejection> {
    let mut response = Response::builder();

    let session = forwarded.1;
    if session.expiration < (Utc::now() + Duration::days(7)) {
        match Session::new(session.userid) {
            Ok(new_session) => match session.location {
                Some(SessionLocation::Cookie) => {
                    response = response.header(
                        "Set-Cookie",
                        format!(
                            "azuma_session={}; Max-Age=2592000; Path=/",
                            new_session.token
                        ),
                    );
                }

                Some(SessionLocation::Header) => {
                    response = response.header("Azuma-Session", format!("{}", new_session.token));
                }

                None => return Err(reject::custom(AzumaRejection::InternalServerError)),
            },
            Err(why) => return Err(reject::custom(why)),
        }
    }

    let original_response = forwarded.0.into_response();
    Ok(response
        .status(original_response.status())
        .body(original_response.into_body()))
}
