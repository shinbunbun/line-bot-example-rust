use std::vec;

use crate::client;

use crate::models::message::text::TextMessage;
use crate::models::message::EachMessageFields;
use crate::models::message::MessageObject;
use crate::{error::AppError, models::webhook_event};
use actix_web::{web, HttpResponse, Responder};

use serde::Serialize;

pub async fn handler(context: web::Json<webhook_event::Root>) -> Result<impl Responder, AppError> {
    println!("{:?}", context);
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
