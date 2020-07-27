use crate::rejection::AzumaRejection;
use chrono::{DateTime, Duration, Utc};
use rsgen::{gen_random_string, OutputCharsType};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub userid: i64,
    pub expiration: DateTime<Utc>,
}

impl Session {
    pub async fn new(userid: i64, pool: &PgPool) -> Result<Session, AzumaRejection> {
        let token = gen_random_string(
            64,
            OutputCharsType::LatinAlphabetAndNumeric {
                use_upper_case: true,
                use_lower_case: true,
            },
        );
        let expiration = Utc::now() + Duration::days(30);

        /*let session = Session {
            token,
            userid,
            expiration: (Utc::now() + Duration::days(30)),
        };*/

        let session: Session = query_as(
            "insert into sessions (token, userid, expiration) values ($1, $2, $3) returning *",
        )
        .bind(token)
        .bind(userid)
        .bind(expiration)
        .fetch_one(pool)
        .await?;
        Ok(session)
    }

    pub async fn get(token: String, pool: &PgPool) -> Result<Session, AzumaRejection> {
        let session: Option<Session> = query_as("select * from sessions where token = $1")
            .bind(token)
            .fetch_optional(pool)
            .await?;

        match session {
            Some(session) => {
                if session.expiration > Utc::now() {
                    Ok(session)
                } else {
                    Err(AzumaRejection::Unauthorized)
                }
            }
            None => Err(AzumaRejection::Unauthorized),
        }
    }
}
