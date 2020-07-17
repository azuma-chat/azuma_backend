use crate::model::{request::LoginCredentials, session::Session, user::User};
use mongodb::Database;
use warp::{reply, Rejection, Reply};

pub async fn registration_handler(
    user: LoginCredentials,
    db: Database,
) -> Result<impl Reply, Rejection> {
    let new_user = User::new(user.name, user.password, &db).await?;
    let session = Session::new(new_user.id, &db).await?;
    Ok(reply::json(&session))
}
