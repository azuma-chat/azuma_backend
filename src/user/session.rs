use crate::{model::session::Session, rejection::AzumaRejection};
use sqlx::PgPool;
use warp::{header, reject, Filter, Rejection};

pub fn with_session(pool: PgPool) -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
    header::header("Authorization")
        .and_then(move |mut authentication_header: String| {
            let db = pool.clone();
            async move {
                let authentication_token = authentication_header.split_off(7);
                if authentication_header == "Bearer " {
                    match Session::get(authentication_token, &db).await {
                        Ok(session) => Ok(session),
                        Err(why) => Err(reject::custom(why)),
                    }
                } else {
                    Err(reject::custom(AzumaRejection::Unauthorized))
                }
            }
        })
        .or_else(|_| async move { Err(reject::custom(AzumaRejection::Unauthorized)) })
}
