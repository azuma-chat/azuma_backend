use crate::{rejection::AzumaRejection, util::to_document::to_doc, AZUMA_DB};
use bson::{doc, from_bson, oid::ObjectId, Bson::Document};
use chrono::{DateTime, Duration, Utc};
use rsgen::{gen_random_string, OutputCharsType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub token: String,
    pub userid: ObjectId,
    pub expiration: DateTime<Utc>,
}

impl Session {
    pub fn new(userid: ObjectId) -> Result<Session, AzumaRejection> {
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

        let coll = AZUMA_DB.collection("sessions");
        match coll.insert_one(to_doc(&session), None) {
            Ok(_) => Ok(session),
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub fn get(token: String) -> Result<Session, AzumaRejection> {
        let coll = AZUMA_DB.collection("sessions");
        match coll.find_one(Some(doc! { "token": token }), None) {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<Session>(Document(doc)) {
                    Ok(session_result) => {
                        if session_result.expiration > Utc::now() {
                            Ok(session_result)
                        } else {
                            Err(AzumaRejection::Unauthorized)
                        }
                    }
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                None => Err(AzumaRejection::Unauthorized),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }
}
