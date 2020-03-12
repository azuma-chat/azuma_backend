use crate::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub status: Option<String>,
}

impl UserResponse {
    pub fn new(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            name: user.name,
            icon: user.icon,
            status: user.status,
        }
    }
}
