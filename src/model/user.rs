use crate::{db::db, rejection::AzumaRejection, util::to_document::to_doc};
use bson::{doc, from_bson, oid::ObjectId, Bson::Document};
use pbkdf2::pbkdf2_simple;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub password: String,
    pub icon: Option<String>,
    pub status: Option<String>,
}

impl User {
    pub async fn new(name: String, password: String) -> Result<User, AzumaRejection> {
        let coll = db().await.collection("users");
        match coll
            .find_one(Some(doc! { "name": name.clone() }), None)
            .await
        {
            Ok(doc) => match doc {
                None => match pbkdf2_simple(&password, 100000) {
                    Ok(hashed_password) => {
                        let user = User {
                            id: ObjectId::new().unwrap(),
                            name,
                            password: hashed_password,
                            icon: None,
                            status: None,
                        };

                        match coll.insert_one(to_doc(&user), None).await {
                            Ok(_) => Ok(user),
                            Err(_) => Err(AzumaRejection::InternalServerError),
                        }
                    }
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                Some(_) => Err(AzumaRejection::AlreadyExists),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub async fn get(name: String) -> Result<User, AzumaRejection> {
        let coll = db().await.collection("users");
        match coll.find_one(Some(doc! { "name": name }), None).await {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<User>(Document(doc)) {
                    Ok(user_result) => Ok(user_result),
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                None => Err(AzumaRejection::NotFound),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }

    pub async fn get_by_id(id: ObjectId) -> Result<User, AzumaRejection> {
        let coll = db().await.collection("users");
        match coll.find_one(Some(doc! { "_id": id }), None).await {
            Ok(doc) => match doc {
                Some(doc) => match from_bson::<User>(Document(doc)) {
                    Ok(user_result) => Ok(user_result),
                    Err(_) => Err(AzumaRejection::InternalServerError),
                },
                None => Err(AzumaRejection::NotFound),
            },
            Err(_) => Err(AzumaRejection::InternalServerError),
        }
    }
}
