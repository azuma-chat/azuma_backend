use crate::model::{response::UserResponse, session::Session, user::User};
use warp::{reject, reply, Rejection, Reply};

pub async fn me_handler(session: Session) -> Result<(impl Reply, Session), Rejection> {
    let user = User::get_by_id(session.userid.clone());
    match user {
        Ok(user) => Ok((reply::json(&UserResponse::new(user)), session)),
        Err(why) => Err(reject::custom(why)),
    }
}
