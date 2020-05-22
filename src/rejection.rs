use bson::DecoderError;
use mongodb::error::Error as MongoError;
use serde::Serialize;
use std::{convert::Infallible, io::Error as IoError};
use warp::{http::StatusCode, reject, reply, Rejection, Reply};

#[derive(Debug)]
pub enum AzumaRejection {
    AlreadyExists,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl reject::Reject for AzumaRejection {}

impl From<DecoderError> for AzumaRejection {
    fn from(_error: DecoderError) -> Self {
        AzumaRejection::InternalServerError
    }
}

impl From<IoError> for AzumaRejection {
    fn from(_error: IoError) -> Self {
        AzumaRejection::InternalServerError
    }
}

impl From<MongoError> for AzumaRejection {
    fn from(_error: MongoError) -> Self {
        AzumaRejection::InternalServerError
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
