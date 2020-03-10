use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginCredentials {
    pub name: String,
    pub password: String,
}
