use std::env::VarError;

use base64::DecodeError;
use hmac::digest::{InvalidLength, MacError};

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Env(VarError),
    BadRequest(String),
    AwcRequestError(awc::error::SendRequestError),
    HmacInvalidLength(InvalidLength),
    HmacVerifyError(MacError),
    Base64Decode(DecodeError),
    SerdeJson(serde_json::Error),
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Internal(_)
            | AppError::Env(_)
            | AppError::AwcRequestError(_)
            | AppError::HmacInvalidLength(_)
            | AppError::HmacVerifyError(_)
            | AppError::Base64Decode(_)
            | AppError::SerdeJson(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        println!("{}", self.to_string());
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Env(errors) => write!(f, "Env var Error: {}", errors),
            AppError::Internal(errors) => write!(f, "Internal Error: {}", errors),
            AppError::BadRequest(errors) => write!(f, "Bad Request: {}", errors),
            AppError::AwcRequestError(errors) => write!(f, "Awc Request Error: {}", errors),
            AppError::HmacInvalidLength(errors) => write!(f, "Hmac length Error: {}", errors),
            AppError::HmacVerifyError(errors) => write!(f, "Hmac verify Error: {}", errors),
            AppError::Base64Decode(errors) => write!(f, "base64 decode error: {}", errors),
            AppError::SerdeJson(errors) => write!(f, "serde json error: {}", errors),
        }
    }
}
