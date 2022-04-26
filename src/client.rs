use actix_http::encoding::Decoder;
use actix_web::{dev::Payload, http::header};
use awc::{Client, ClientResponse};

use crate::{config, error::AppError};

pub async fn line_post_request<T: serde::Serialize>(
    body: T,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, AppError> {
    Client::new()
        .post(url)
        .insert_header((
            header::AUTHORIZATION,
            format!(
                "{}{}",
                "Bearer ".to_string(),
                config::get_token().map_err(AppError::Env)?
            ),
        ))
        .send_json(&body)
        .await
        .map_err(AppError::AwcRequestError)
}
