pub mod webhook;

use actix_http::{encoding::Decoder, header, Payload};
use awc::ClientResponse;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

use crate::{
    error::Error,
    models::{message::MessageObject, profile::Profile},
};

pub static API_ENDPOINT_BASE: &str = "https://api.line.me";

pub struct Client {
    channel_access_token: String,
    channel_secret: String,
}

impl Client {
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
    async fn get<T: Serialize>(
        &self,
        url: &str,
        query: Option<&T>,
    ) -> Result<ClientResponse<Decoder<Payload>>, Error> {
        let url = match query {
            Some(query) => {
                let query =
                    serde_urlencoded::to_string(query).map_err(Error::SerdeUrlEncodedError)?;
                format!("{}?{}", url, query)
            }
            None => url.to_string(),
        };
        let mut response = awc::Client::new()
            .get(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send()
            .await
            .map_err(Error::AwcSendRequestError)?;
        if response.status() != 200 {
            let res_body = response.body().await.map_err(Error::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(Error::FromUtf8Error)?;
            return Err(Error::AWCClientError(res_body, "".to_string()));
        }
        Ok(response)
    }
    async fn post<T: serde::Serialize>(
        &self,
        body: T,
        url: &str,
    ) -> Result<ClientResponse<Decoder<Payload>>, Error> {
        let json = serde_json::to_string(&body).expect("json encode error");
        let mut response = awc::Client::new()
            .post(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send_json(&body)
            .await
            .map_err(Error::AwcSendRequestError)?;
        if response.status() != 200 {
            let res_body = response.body().await.map_err(Error::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(Error::FromUtf8Error)?;
            return Err(Error::AWCClientError(res_body, json));
        }
        Ok(response)
    }

    #[allow(clippy::all)]
    pub async fn put<T: serde::Serialize>(
        &self,
        body: T,
        url: &str,
    ) -> Result<ClientResponse<Decoder<Payload>>, Error> {
        let json = serde_json::to_string(&body).expect("json encode error");
        let mut response = awc::Client::new()
            .put(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send_json(&body)
            .await
            .map_err(Error::AwcSendRequestError)?;
        if response.status() != 200 {
            let res_body = response.body().await.map_err(Error::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(Error::FromUtf8Error)?;
            return Err(Error::AWCClientError(res_body, json));
        }
        Ok(response)
    }

    pub async fn delete<T: Serialize>(
        &self,
        url: &str,
        query: Option<&T>,
    ) -> Result<ClientResponse<Decoder<Payload>>, Error> {
        let url = match query {
            Some(query) => {
                let query =
                    serde_urlencoded::to_string(query).map_err(Error::SerdeUrlEncodedError)?;
                format!("{}?{}", url, query)
            }
            None => url.to_string(),
        };
        let mut response = awc::Client::new()
            .delete(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send()
            .await
            .map_err(Error::AwcSendRequestError)?;
        if response.status() != 200 {
            let res_body = response.body().await.map_err(Error::ActixWebPayloadError)?;
            let res_body = String::from_utf8(res_body.to_vec()).map_err(Error::FromUtf8Error)?;
            return Err(Error::AWCClientError(res_body, "".to_string()));
        }
        Ok(response)
    }

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
    pub async fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> Result<(), Error> {
        let body = ReplyMessage {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        self.post(body, &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE))
            .await?;
        Ok(())
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<Profile, Error> {
        let url = format!("{}/v2/bot/profile/{}", API_ENDPOINT_BASE, user_id);
        let mut res = self.get(&url, None::<&[(); 0]>).await?;
        let res_body = res
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
    }

    pub async fn get_content(&self, message_id: &str) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}/v2/bot/message/{}/content",
            API_ENDPOINT_BASE, message_id
        );
        let mut res = self.get(&url, None::<&[(); 0]>).await?;
        let res_body = res
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        Ok(res_body)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyMessage {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}
