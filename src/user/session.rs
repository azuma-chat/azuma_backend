use crate::{rejection::AzumaRejection, AZUMADB};
use bson::{bson, doc, from_bson, to_bson, Bson::Document};
use chrono::{DateTime, Duration, Utc};
use rsgen::{gen_random_string, OutputCharsType};
use serde::{Deserialize, Serialize};
use warp::{cookie, http::Response, reject, Filter, Rejection, Reply};

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub token: String,
    pub userid: i64,
    pub expiration: DateTime<Utc>,
}

pub fn with_session() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    cookie::optional("azuma_session").and_then(|session_cookie| async move {
        match session_cookie {
            Some(session_cookie) => match get_session(session_cookie) {
                Ok(session) => match validate_session(session) {
                    Ok(session) => Ok(session),
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
        match new_session(session.userid) {
            Ok(token) => {
                response = response.header(
                    "Set-Cookie",
                    format!("azuma_session={}; Max-Age=2592000; Path=/", token),
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

fn get_session(token: String) -> Result<Session, AzumaRejection> {
    let coll = AZUMADB.collection("sessions");
    match coll.find_one(Some(doc! { "token": token }), None) {
        Ok(doc) => match doc {
            Some(doc) => match from_bson::<Session>(Document(doc)) {
                Ok(session_result) => Ok(session_result),
                Err(_) => Err(AzumaRejection::Unauthorized),
            },
            None => Err(AzumaRejection::Unauthorized),
        },
        Err(_) => Err(AzumaRejection::InternalServerError),
    }
}

fn validate_session(session: Session) -> Result<Session, AzumaRejection> {
    if session.expiration > Utc::now() {
        Ok(session)
    } else {
        Err(AzumaRejection::Unauthorized)
    }
}

pub fn new_session(userid: i64) -> Result<String, AzumaRejection> {
    let token = gen_random_string(
        64,
        OutputCharsType::LatinAlphabetAndNumeric {
            use_upper_case: true,
            use_lower_case: true,
        },
    );

    let session = Session {
        token,
        userid,
        expiration: (Utc::now() + Duration::days(30)),
    };

    let coll = AZUMADB.collection("sessions");
    match coll.insert_one(
        to_bson(&session).unwrap().as_document().unwrap().clone(),
        None,
    ) {
        Ok(_) => Ok(session.token),
        Err(_) => Err(AzumaRejection::InternalServerError),
    }
}
