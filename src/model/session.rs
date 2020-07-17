use crate::{rejection::AzumaRejection, util::to_document::to_doc};
use chrono::{DateTime, Duration, Utc};
use mongodb::bson::{doc, from_bson, oid::ObjectId, Bson::Document};
use mongodb::Database;
use rsgen::{gen_random_string, OutputCharsType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub token: String,
    pub userid: ObjectId,
    pub expiration: DateTime<Utc>,
}

impl Session {
    pub async fn new(userid: ObjectId, db: &Database) -> Result<Session, AzumaRejection> {
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

        let coll = db.collection("sessions");
        coll.insert_one(to_doc(&session), None).await?;
        Ok(session)
    }

    pub async fn get(token: String, db: &Database) -> Result<Session, AzumaRejection> {
        let coll = db.collection("sessions");
        let session = coll.find_one(Some(doc! { "token": token }), None).await?;
        match session {
            Some(session) => {
                let session = from_bson::<Session>(Document(session))?;
                if session.expiration > Utc::now() {
                    Ok(session)
                } else {
                    Err(AzumaRejection::Unauthorized)
                }
            }
            None => Err(AzumaRejection::Unauthorized),
        }
    }
}
