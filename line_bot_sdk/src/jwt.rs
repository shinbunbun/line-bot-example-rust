use std::time::{self, SystemTime};

use crate::Error;
use josekit::{
    jwk::Jwk,
    jws::{JwsHeader, RS256},
    jwt::{self, JwtPayload},
};
use serde_json::json;

pub fn create_jwt(kid: &str, channel_id: &str, private_key: &str) -> Result<String, Error> {
    let mut header = JwsHeader::new();
    header.set_token_type("JWT");
    header.set_algorithm("RS256");
    header.set_key_id(kid);

    let mut payload = JwtPayload::new();
    payload.set_issuer(channel_id);
    payload.set_subject(channel_id);
    payload.set_audience(vec!["https://api.line.me/"]);
    payload.set_expires_at(&(SystemTime::now() + time::Duration::from_secs(60 * 30)));
    payload
        .set_claim("token_exp", Some(json!((60 * 60 * 24 * 30).to_string())))
        .map_err(Error::JoseError)?;

    // Signing JWT
    let jwk = Jwk::from_bytes(private_key.as_bytes()).map_err(Error::JoseError)?;
    let signer = RS256.signer_from_jwk(&jwk).map_err(Error::JoseError)?;
    jwt::encode_with_signer(&payload, &header, &signer).map_err(Error::JoseError)
}

#[cfg(test)]
mod test {
    use std::env;

    use super::create_jwt;

    #[test]
    fn test_create_jwt() {
        let kid = env::var("JWT_TEST_KID").unwrap();
        let channel_id = env::var("CHANNEL_ID").unwrap();
        let private_key = env::var("JWT_PRIVATE_KEY").unwrap();
        create_jwt(&kid, &channel_id, &private_key).unwrap();
    }
}
