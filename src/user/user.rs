use crate::model::{response::UserResponse, session::Session, user::User};
use mongodb::Database;
use warp::{reply, Rejection, Reply};

pub async fn me_handler(
    session: Session,
    db: Database,
) -> Result<(impl Reply, Session, Database), Rejection> {
    let user = User::get_by_id(session.userid.clone(), &db).await?;
    Ok((reply::json(&UserResponse::new(user)), session, db))
}
