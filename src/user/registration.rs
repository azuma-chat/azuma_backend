use crate::model::{request::LoginCredentials, session::Session, user::User};
use sqlx::PgPool;
use warp::{reply, Rejection, Reply};

pub async fn registration_handler(
    user: LoginCredentials,
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let new_user = User::new(user.name, user.password, &pool).await?;
    let session = Session::new(new_user.id, &pool).await?;
    Ok(reply::json(&session))
}
