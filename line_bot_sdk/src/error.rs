#[derive(Debug)]
pub enum Error {
    SerdeJsonError(serde_json::Error),
    HmacDijestInvalidLength(hmac::digest::InvalidLength),
    Base64DecodeError(base64::DecodeError),
    HmacDigestMacError(hmac::digest::MacError),
    ActixWebPayloadError(actix_web::error::PayloadError),
    AwcSendRequestError(awc::error::SendRequestError),
    FromUtf8Error(std::string::FromUtf8Error),
    AWCClientError(String, String),
    SerdeUrlEncodedError(serde_urlencoded::ser::Error),
    JoseError(josekit::JoseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerdeJsonError(errors) => write!(f, "serde_json error: {}", errors),
            Error::HmacDijestInvalidLength(errors) => {
                write!(f, "hmac dijest InvalidLength: {}", errors)
            }
            Error::Base64DecodeError(errors) => write!(f, "base64 DecodeError: {}", errors),
            Error::HmacDigestMacError(errors) => write!(f, "hmac digest MacError: {}", errors),
            Error::ActixWebPayloadError(errors) => {
                write!(f, "actix_web error PayloadError: {}", errors)
            }
            Error::AwcSendRequestError(errors) => {
                write!(f, "awc error SnedRequest error: {}", errors)
            }
            Error::FromUtf8Error(errors) => write!(f, "std string FromUtf8Error: {}", errors),
            Error::AWCClientError(errors, request_body) => write!(
                f,
                "AWC client error: {}\nRequest body: {}",
                errors, request_body
            ),
            Error::SerdeUrlEncodedError(errors) => write!(f, "serde_urlencoded error: {}", errors),
            Error::JoseError(errors) => write!(f, "jose error: {}", errors),
        }
    }
}
