use actix_http::{encoding::Decoder, header, Payload};
use awc::ClientResponse;
use hmac::{Hmac, Mac};
use log::info;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{error::AppError, models::message::MessageObject};

pub static API_ENDPOINT_BASE: &str = "https://api.line.me";

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
    pub fn verify_signature(&self, signature: &str, context: &str) -> Result<(), AppError> {
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
            return Err(AppError::AWCClientError(res_body));
        }
        Ok(response)
    }
    async fn line_get_request(
        &self,
        url: &str,
    ) -> Result<ClientResponse<Decoder<Payload>>, AppError> {
        let mut response = awc::Client::new()
            .get(url)
            .insert_header((
                header::AUTHORIZATION,
                format!(
                    "{}{}",
                    "Bearer ".to_string(),
                    &self.get_channel_access_token()
                ),
            ))
            .send()
            .await
            .map_err(AppError::AwcRequestError)?;
        if response.status() != 200 {
            let res_body = response
                .body()
                .await
                .map_err(AppError::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(AppError::FromUtf8Error)?;
            return Err(AppError::AWCClientError(res_body));
        }
        Ok(response)
    }
    pub async fn get_profile(&self, user_id: &str) -> Result<Profile, AppError> {
        let url = format!("{}/v2/bot/profile/{}", API_ENDPOINT_BASE, user_id);
        let mut res = self.line_get_request(&url).await?;
        let res_body = res
            .body()
            .await
            .map_err(AppError::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(AppError::SerdeJson)
    }
    pub async fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> Result<(), AppError> {
        let body = ReplyMessage {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        self.line_post_request(body, &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE))
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub display_name: String,
    pub user_id: String,
    pub language: String,
    pub picture_url: String,
    pub status_message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyMessage {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}
