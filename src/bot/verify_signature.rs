use hmac::Hmac;
use hmac::Mac;
use sha2::Sha256;

use crate::{config, error::AppError};

pub fn verify(signature: &str, context: String) -> Result<(), AppError> {
    type HmacSha256 = Hmac<Sha256>;
    let secret = config::get_secret().map_err(AppError::Env)?;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).map_err(AppError::HmacInvalidLength)?;
    mac.update(context.as_bytes());

    let x_line_signature = base64::decode(signature).map_err(AppError::Base64Decode)?;
    mac.verify_slice(&x_line_signature[..])
        .map_err(AppError::HmacVerifyError)
}
