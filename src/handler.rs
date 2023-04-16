use std::sync::Arc;

use actix_web::rt::spawn;
use line_bot_sdk::Client;
use log::error;

use actix_web::{web, HttpResponse, Responder};
use line_bot_sdk::extractor::CustomHeader;
use line_bot_sdk::models::message::MessageObject;
use line_bot_sdk::models::webhook_event;

use serde::{Deserialize, Serialize};

use crate::event::{
    follow, join, leave, member_joined, member_left, message, postback, unfollow, unsend,
};
use crate::AppState;

pub async fn handler(
    context: String,
    custom_header: CustomHeader,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let client = Arc::clone(&app_state.line_client);

    let signature = custom_header.x_line_signature;

    if let Err(err) = client.verify_signature(&signature, &context) {
        error!("Invalid signature: {}", err);
    };

    let webhook_event = match serde_json::from_str(&context) {
        Ok(event) => event,
        Err(err) => {
            error!("Invalid request body: {}", err);
            return HttpResponse::Ok().body("");
        }
    };

    spawn(async move { webhook_handler(&webhook_event, &client).await });

    HttpResponse::Ok().body("")
}

async fn webhook_handler(context: &webhook_event::Root, client: &Client) {
    for event in &context.events {
        let event_type_handler_response = match event.type_field.as_str() {
            "message" => message::index(client, event).await,
            "unsend" => unsend::index(event).await,
            "postback" => postback::index(event).await,
            "join" => join::index().await,
            "leave" => leave::index().await,
            "follow" => follow::index().await,
            "unfollow" => unfollow::index(event).await,
            "memberJoined" => member_joined::index(event).await,
            "memberLeft" => member_left::index(event).await,
            _ => {
                error!("Unknown event type: {}", event.type_field);
                return;
            }
        };

        let reply_messages = match event_type_handler_response {
            Ok(reply_message) => reply_message,
            Err(e) => {
                error!("Error: {}", e);
                return;
            }
        };

        if let Some(reply_messages) = reply_messages {
            let reply_token = match event.reply_token.as_ref() {
                Some(reply_token) => reply_token,
                None => {
                    error!("Reply token is not found");
                    return;
                }
            };

            if let Err(err) = client.reply(reply_token, reply_messages, None).await {
                error!("Error: {}", err);
                return;
            };
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplyMessage {
    #[serde(rename(serialize = "replyToken"))]
    reply_token: String,
    messages: Vec<MessageObject>,
}
