use crate::{
    model::{request::LoginCredentials, session::Session, user::User},
    rejection::AzumaRejection,
};
use pbkdf2::pbkdf2_check;
use warp::{reject, reply, Rejection, Reply};

pub async fn login_handler(user: LoginCredentials) -> Result<impl Reply, Rejection> {
    match User::get(user.name).await {
        Ok(db_user) => match pbkdf2_check(&user.password, &db_user.password) {
            Ok(_) => match Session::new(db_user.id).await {
                Ok(session) => Ok(reply::json(&session)),
                Err(why) => Err(reject::custom(why)),
            },
            Err(_) => Err(reject::custom(AzumaRejection::Unauthorized)),
        },
        Err(why) => Err(reject::custom(why)),
    }
}
