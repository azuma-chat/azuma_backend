use crate::{rejection::AzumaRejection, AZUMADB};
use bson::{bson, doc, from_bson, Bson::Document};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use warp::{cookie, reject, Filter, Rejection};

pub struct Session {
    pub userid: i64,
}

#[derive(Deserialize, Debug)]
struct SessionResult {
    token: String,
    userid: i64,
    expiration: DateTime<Utc>,
}

pub fn session_middleware() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    cookie::optional("azuma_session").and_then(|session_cookie| async move {
        match session_cookie {
            Some(session_cookie) => match get_session(session_cookie) {
                Ok(session_result) => match validate_session(session_result) {
                    Ok(session_result) => Ok(Session {
                        userid: session_result.userid,
                    }),
                    Err(why) => Err(reject::custom(why)),
                },
                Err(why) => Err(reject::custom(why)),
            },
            None => Err(reject::custom(AzumaRejection::Unauthorized)),
        }
    })
}

fn get_session(token: String) -> Result<SessionResult, AzumaRejection> {
    let coll = AZUMADB.collection("sessions");
    match coll.find_one(Some(doc! {"token": token}), None) {
        Ok(doc) => match doc {
            Some(doc) => match from_bson::<SessionResult>(Document(doc)) {
                Ok(session_result) => Ok(session_result),
                Err(_) => Err(AzumaRejection::Unauthorized),
            },
            None => Err(AzumaRejection::Unauthorized),
        },
        Err(_) => Err(AzumaRejection::InternalServerError),
    }
}

fn validate_session(session: SessionResult) -> Result<SessionResult, AzumaRejection> {
    if session.expiration > Utc::now() {
        Ok(session)
    } else {
        Err(AzumaRejection::Unauthorized)
    }
}
