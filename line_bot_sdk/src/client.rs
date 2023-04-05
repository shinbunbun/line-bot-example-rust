pub mod content;
pub mod message;
pub mod signature;
pub mod token;
pub mod user;
pub mod webhook;

use actix_http::header;
use awc::SendClientRequest;
use serde::Serialize;

use crate::{awc_wrapper::SendClientRequestFut, error::Error};

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

    fn get<T: Serialize>(
        &self,
        url: &str,
        query: Option<&T>,
        content_type: Option<&str>,
        with_authorization: bool,
    ) -> Result<SendClientRequest, Error> {
        let url = match query {
            Some(query) => {
                let query =
                    serde_urlencoded::to_string(query).map_err(Error::SerdeUrlEncodedError)?;
                format!("{}?{}", url, query)
            }
            None => url.to_string(),
        };
        let mut request = awc::Client::new().get(url);
        if with_authorization {
            request = request.insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ));
        }
        if let Some(content_type) = content_type {
            request = request.content_type(content_type);
        }
        Ok(request.send())
    }

    fn post<T: serde::Serialize>(
        &self,
        body: T,
        url: &str,
        x_line_retry_key: Option<&str>,
    ) -> Result<SendClientRequest, Error> {
        let mut request = awc::Client::new().post(url).insert_header((
            header::AUTHORIZATION,
            format!("{}{}", "Bearer ", self.get_channel_access_token()),
        ));
        if let Some(x_line_retry_key) = x_line_retry_key {
            request = request.insert_header((
                header::HeaderName::from_static("X-Line-Retry-Key"),
                x_line_retry_key,
            ));
        }
        Ok(request.send_json(&body))
    }

    fn post_form<T: serde::Serialize>(
        &self,
        body: T,
        url: &str,
    ) -> Result<SendClientRequest, Error> {
        let request = awc::Client::new().post(url).send_form(&body);

        Ok(request)
    }

    fn put<T: serde::Serialize>(&self, body: T, url: &str) -> Result<SendClientRequest, Error> {
        let request = awc::Client::new()
            .put(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send_json(&body);
        Ok(request)
    }

    pub async fn delete<T: Serialize>(
        &self,
        url: &str,
        query: Option<&T>,
    ) -> Result<SendClientRequest, Error> {
        let url = match query {
            Some(query) => {
                let query =
                    serde_urlencoded::to_string(query).map_err(Error::SerdeUrlEncodedError)?;
                format!("{}?{}", url, query)
            }
            None => url.to_string(),
        };
        let request = awc::Client::new()
            .delete(url)
            .insert_header((
                header::AUTHORIZATION,
                format!("{}{}", "Bearer ", self.get_channel_access_token()),
            ))
            .send();
        Ok(request)
    }
}
