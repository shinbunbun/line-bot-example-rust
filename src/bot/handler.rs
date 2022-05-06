use std::vec;

use crate::client;

use crate::config;
use crate::extractor::CustomHeader;
use crate::models;
use crate::models::message::text::TextMessage;
use crate::models::message::EachMessageFields;
use crate::models::message::MessageObject;
use crate::models::webhook_event::Event;
use crate::{error::AppError, models::webhook_event};
use actix_http::RequestHead;
use actix_web::{http::header, web, HttpResponse, Responder};

use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;

pub async fn handler(
    context: String,
    custom_header: CustomHeader,
) -> Result<impl Responder, AppError> {
    println!("{:?}", context);
    type HmacSha256 = Hmac<Sha256>;
    let secret = config::get_secret().map_err(AppError::Env)?;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).map_err(AppError::HmacInvalidLength)?;
    mac.update(context.as_bytes());
    println!("header: {:#?}", custom_header);

    let x_line_signature =
        base64::decode(custom_header.x_line_signature).map_err(AppError::Base64Decode)?;
    mac.verify_slice(&x_line_signature[..])
        .map_err(AppError::HmacVerifyError)?;

    let context: webhook_event::Root =
        serde_json::from_str(context.as_str()).map_err(AppError::SerdeJson)?;

    for event in &context.events {
        let message = event
            .message
            .as_ref()
            .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
        println!("{:?}", message.text);

        let reply_messages = ReplyMessage {
            reply_token: event
                .reply_token
                .as_ref()
                .ok_or_else(|| AppError::BadRequest("Reply token not found".to_string()))?
                .to_string(),
            messages: vec![{
                MessageObject {
                    quick_reply: None,
                    sender: None,
                    message: EachMessageFields::Text(TextMessage {
                        text: message.text.clone(),
                        type_field: "text".to_string(),
                        emojis: None,
                    }),
                }
            }],
        };

        println!("{:?}", &reply_messages);

        let mut response =
            client::line_post_request(reply_messages, "https://api.line.me/v2/bot/message/reply")
                .await?;

        println!("{:?}", response.body().await.unwrap());
    }
    return Ok(HttpResponse::Ok().json("Ok"));
}

#[derive(Debug, Serialize)]
struct ReplyMessage {
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    messages: Vec<MessageObject>,
}
