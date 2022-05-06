use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    Error, FromRequest,
};
use futures::future::{err, ok, Ready};

#[derive(Debug)]
pub struct CustomHeader {
    pub x_line_signature: String,
}

impl FromRequest for CustomHeader {
    type Error = Error;

    type Future = Ready<Result<Self, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_http::Payload,
    ) -> Self::Future {
        let x_line_signature = req.headers().get("x-line-signature");
        let x_line_signature = match x_line_signature {
            Some(x) => x.to_str(),
            None => return err(ErrorBadRequest("x-line-signature not found")),
        };
        let x_line_signature = match x_line_signature {
            Ok(x) => x,
            Err(e) => return err(ErrorInternalServerError(e)),
        };
        let custom_header = CustomHeader {
            x_line_signature: x_line_signature.to_string(),
        };
        ok(custom_header)
    }
}
