use crate::model::{user::User, permission::UserPermission};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub icon: Option<String>,
    pub status: Option<String>,
    pub permissions: UserPermission,
}

impl UserResponse {
    pub fn new(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
            icon: user.icon,
            status: user.status,
            permissions: user.permissions,
        }
    }
}
