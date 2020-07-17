use crate::{db::db, rejection::AzumaRejection, util::to_document::to_doc};
use mongodb::bson::{doc, from_bson, oid::ObjectId, Bson::Document};
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
        let user = coll
            .find_one(Some(doc! { "name": name.clone() }), None)
            .await?;
        match user {
            None => {
                let hashed_password = pbkdf2_simple(&password, 100000)?;
                let user = User {
                    id: ObjectId::new(),
                    name,
                    password: hashed_password,
                    icon: None,
                    status: None,
                };
                coll.insert_one(to_doc(&user), None).await?;
                Ok(user)
            }
            Some(_) => Err(AzumaRejection::AlreadyExists),
        }
    }

    pub async fn get(name: String) -> Result<User, AzumaRejection> {
        let coll = db().await.collection("users");
        let user = coll.find_one(Some(doc! { "name": name }), None).await?;
        match user {
            Some(user) => {
                let user = from_bson::<User>(Document(user))?;
                Ok(user)
            }
            None => Err(AzumaRejection::NotFound),
        }
    }

    pub async fn get_by_id(id: ObjectId) -> Result<User, AzumaRejection> {
        let coll = db().await.collection("users");
        let user = coll.find_one(Some(doc! { "_id": id }), None).await?;
        match user {
            Some(user) => {
                let user = from_bson::<User>(Document(user))?;
                Ok(user)
            }
            None => Err(AzumaRejection::NotFound),
        }
    }
}
