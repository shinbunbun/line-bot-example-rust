use actix_http::{encoding::Decoder, header, Payload};
use awc::ClientResponse;
use hmac::{Hmac, Mac};
use log::info;
use sha2::Sha256;

use crate::error::AppError;

impl super::Client {
    pub fn new(channel_access_token: String, channel_secret: String) -> Self {
        Self {
            channel_access_token,
            channel_secret,
        }
    }
    pub fn get_channel_access_token(&self) -> &str {
        &self.channel_access_token
    }
    pub fn get_channel_secret(&self) -> &str {
        &self.channel_secret
    }
    pub fn verify(&self, signature: &str, context: &str) -> Result<(), AppError> {
        type HmacSha256 = Hmac<Sha256>;
        let secret = self.get_channel_secret();
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).map_err(AppError::HmacInvalidLength)?;
        mac.update(context.as_bytes());

        let x_line_signature = base64::decode(signature).map_err(AppError::Base64Decode)?;
        mac.verify_slice(&x_line_signature[..])
            .map_err(AppError::HmacVerifyError)
    }
    pub async fn line_post_request<T: serde::Serialize>(
        &self,
        body: T,
        url: &str,
    ) -> Result<ClientResponse<Decoder<Payload>>, AppError> {
        let json = serde_json::to_string(&body).expect("json encode error");
        info!("{}", json);
        let mut response = awc::Client::new()
            .post(url)
            .insert_header((
                header::AUTHORIZATION,
                format!(
                    "{}{}",
                    "Bearer ".to_string(),
                    &self.get_channel_access_token()
                ),
            ))
            .send_json(&body)
            .await
            .map_err(AppError::AwcRequestError)?;
        if response.status() != 200 {
            let res_body = response
                .body()
                .await
                .map_err(AppError::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(AppError::FromUtf8Error)?;
            return Err(AppError::LINEReplyError(res_body));
        }
        Ok(response)
    }
}
