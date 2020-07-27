use crate::model::{response::UserResponse, session::Session, user::User};
use sqlx::PgPool;
use warp::{reply, Rejection, Reply};

pub async fn me_handler(session: Session, pool: PgPool) -> Result<(impl Reply,), Rejection> {
    let user = User::get_by_id(session.userid, &pool).await?;
    Ok((reply::json(&UserResponse::new(user)),))
}
