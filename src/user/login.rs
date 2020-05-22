use crate::{
    model::{request::LoginCredentials, session::Session, user::User},
    rejection::AzumaRejection,
};
use pbkdf2::pbkdf2_check;
use warp::{reply, Rejection, Reply};

pub async fn login_handler(user: LoginCredentials) -> Result<impl Reply, Rejection> {
    let db_user = User::get(user.name).await?;
    match pbkdf2_check(&user.password, &db_user.password) {
        Ok(_) => {
            let session = Session::new(db_user.id).await?;
            Ok(reply::json(&session))
        }
        Err(_) => Err(AzumaRejection::Unauthorized.into()),
    }
}
