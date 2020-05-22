use crate::model::{request::LoginCredentials, session::Session, user::User};
use warp::{reply, Rejection, Reply};

pub async fn registration_handler(user: LoginCredentials) -> Result<impl Reply, Rejection> {
    let new_user = User::new(user.name, user.password).await?;
    let session = Session::new(new_user.id).await?;
    Ok(reply::json(&session))
}
