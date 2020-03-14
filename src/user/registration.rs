use crate::model::{request::LoginCredentials, session::Session, user::User};
use warp::{reject, reply, Rejection, Reply};

pub async fn registration_handler(user: LoginCredentials) -> Result<impl Reply, Rejection> {
    match User::new(user.name, user.password) {
        Ok(new_user) => match Session::new(new_user.id) {
            Ok(session) => Ok(reply::json(&session)),
            Err(why) => Err(reject::custom(why)),
        },
        Err(why) => Err(reject::custom(why)),
    }
}
