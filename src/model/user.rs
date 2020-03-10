use crate::{rejection::AzumaRejection, AZUMA_DB};
use bson::{bson, doc, from_bson, Bson::Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub icon: String,
    pub status: String,
}

impl User {
    pub fn get(name: String) -> Result<User, AzumaRejection> {
        let coll = AZUMA_DB.collection("users");
        match coll.find_one(Some(doc! { "name": name }), None) {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<User>(Document(doc)) {
                    Ok(user_result) => Ok(user_result),
                    Err(_) => Err(AzumaRejection::Unauthorized),
                },
                None => Err(AzumaRejection::NotFound),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }
}
