use std::vec;

use crate::config;
use crate::{error::AppError, models::webhook_event_object};
use actix_web::{http::header, web, HttpResponse, Responder};
use awc::Client;
use serde::Serialize;

pub async fn handler(
    context: web::Json<webhook_event_object::Root>,
) -> Result<impl Responder, AppError> {
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
                Message {
                    r#type: "text".to_string(),
                    text: message.text.to_string(),
                }
            }],
        };

        println!("{:?}", &reply_messages);

        let mut response = Client::new()
            .post("https://api.line.me/v2/bot/message/reply")
            .insert_header((
                header::AUTHORIZATION,
                format!(
                    "{}{}",
                    "Bearer ".to_string(),
                    config::get_token().map_err(AppError::Env)?
                ),
            ))
            .send_json(&reply_messages)
            .await
            .map_err(AppError::AwcRequestError)?;

        println!("{:?}", response.body().await.unwrap());
    }
    return Ok(HttpResponse::Ok().json("Ok"));
}

#[derive(Debug, Serialize)]
struct ReplyMessage {
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct Message {
    r#type: String,
    text: String,
}
