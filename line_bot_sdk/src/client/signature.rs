use crate::error::Error;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::Client;

impl Client {
    pub fn verify_signature(&self, signature: &str, context: &str) -> Result<(), Error> {
        type HmacSha256 = Hmac<Sha256>;
        let secret = self.get_channel_secret();
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(Error::HmacDijestInvalidLength)?;
        mac.update(context.as_bytes());

        let x_line_signature = base64::decode(signature).map_err(Error::Base64DecodeError)?;
        mac.verify_slice(&x_line_signature[..])
            .map_err(Error::HmacDigestMacError)
    }
}
