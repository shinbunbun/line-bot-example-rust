use line_bot_sdk::client::Client;
use log::info;
use std::vec;

use actix_web::{HttpResponse, Responder};
use line_bot_sdk::extractor::CustomHeader;
use line_bot_sdk::models::message::text::TextMessage;
use line_bot_sdk::models::message::EachMessageFields;
use line_bot_sdk::models::message::MessageObject;
use line_bot_sdk::{error::AppError, models::webhook_event};

use serde::Serialize;

use crate::config;

async fn webhook_handler(
    context: webhook_event::Root,
    client: Client,
) -> Result<HttpResponse, AppError> {
    for event in &context.events {
        let message = event
            .message
            .as_ref()
            .ok_or_else(|| AppError::BadRequest("Message not found".to_string()))?;
        let reply_token = event
            .reply_token
            .as_ref()
            .ok_or_else(|| AppError::BadRequest("Reply token not found".to_string()))?
            .as_str();
        let reply_messages = vec![{
            MessageObject {
                quick_reply: None,
                sender: None,
                message: EachMessageFields::Text(TextMessage {
                    text: message.text.clone(),
                    type_field: "text".to_string(),
                    emojis: None,
                }),
            }
        }];

        client.reply(reply_token, reply_messages, None).await?;
    }
    return Ok(HttpResponse::Ok().json("Ok"));
}

pub async fn handler(
    context: String,
    custom_header: CustomHeader,
) -> Result<impl Responder, AppError> {
    info!("Request body: {}", context);

    let client = Client::new(
        config::get_token().map_err(AppError::Env)?,
        config::get_secret().map_err(AppError::Env)?,
    );

    let signature = custom_header.x_line_signature.as_str();
    client.verify(signature, context.as_str())?;

    let context: webhook_event::Root =
        serde_json::from_str(context.as_str()).map_err(AppError::SerdeJson)?;
    webhook_handler(context, client).await
}

#[derive(Debug, Serialize)]
struct ReplyMessage {
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    messages: Vec<MessageObject>,
}
