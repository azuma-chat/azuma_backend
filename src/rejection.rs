use serde::Serialize;
use sqlx::Error as SqlxError;
use std::{convert::Infallible, io::Error as IoError};
use warp::{
    http::StatusCode,
    reject::{self, MethodNotAllowed, UnsupportedMediaType},
    reply, Rejection, Reply,
};

#[derive(Debug)]
pub enum AzumaRejection {
    AlreadyExists,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl reject::Reject for AzumaRejection {}

impl From<IoError> for AzumaRejection {
    fn from(_error: IoError) -> Self {
        AzumaRejection::InternalServerError
    }
}

impl From<SqlxError> for AzumaRejection {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => AzumaRejection::NotFound,
            _ => AzumaRejection::InternalServerError,
        }
    }
}

impl From<AzumaRejection> for Rejection {
    fn from(rejection: AzumaRejection) -> Self {
        reject::custom(rejection)
    }
}

#[derive(Serialize)]
pub struct RejectionMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(rej: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    if rej.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(AzumaRejection::AlreadyExists) = rej.find() {
        code = StatusCode::BAD_REQUEST;
        message = "ALREADY_EXISTS";
    } else if let Some(AzumaRejection::InternalServerError) = rej.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    } else if let Some(AzumaRejection::NotFound) = rej.find() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(AzumaRejection::Unauthorized) = rej.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "UNAUTHORIZED";
    } else if let Some(_) = rej.find::<UnsupportedMediaType>() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = "UNSUPPORTED_MEDIA_TYPE";
    } else if let Some(_) = rej.find::<MethodNotAllowed>() {
        // method not allowed is usually returned by at least one filter, so it is often in the rejection and
        // is thus at the end of the rejection chain so in case other rejections are there, these will be returned
        // due to how the warp-filter-rejection-chain works, not found is also returned as method not allowed
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED_OR_NOT_FOUND";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = reply::json(&RejectionMessage {
        code: code.as_u16(),
        message: message.to_string(),
    });
    Ok(reply::with_status(json, code))
}
