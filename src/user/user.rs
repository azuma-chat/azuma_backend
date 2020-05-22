use crate::model::{response::UserResponse, session::Session, user::User};
use warp::{reply, Rejection, Reply};

pub async fn me_handler(session: Session) -> Result<(impl Reply, Session), Rejection> {
    let user = User::get_by_id(session.userid.clone()).await?;
    Ok((reply::json(&UserResponse::new(user)), session))
}
