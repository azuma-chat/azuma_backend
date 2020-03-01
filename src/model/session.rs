use crate::{rejection::AzumaRejection, AZUMADB};
use bson::{bson, doc, from_bson, to_bson, Bson::Document};
use chrono::{DateTime, Duration, Utc};
use rsgen::{gen_random_string, OutputCharsType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub token: String,
    pub userid: i64,
    pub expiration: DateTime<Utc>,
}

impl Session {
    pub fn new(userid: i64) -> Result<Session, AzumaRejection> {
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
            Ok(_) => Ok(session),
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub fn get(token: String) -> Result<Session, AzumaRejection> {
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

    pub fn validate(&self) -> Result<(), AzumaRejection> {
        if self.expiration > Utc::now() {
            Ok(())
        } else {
            Err(AzumaRejection::Unauthorized)
        }
    }
}
