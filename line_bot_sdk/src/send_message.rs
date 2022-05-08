pub static API_ENDPOINT_BASE: &str = "https://api.line.me";

use serde::Serialize;

use crate::{api_request, error::AppError, models::message::MessageObject};

pub async fn reply(
    reply_token: String,
    messages: Vec<MessageObject>,
    notification_disabled: Option<bool>,
) -> Result<(), AppError> {
    let body = ReplyMessage {
        reply_token: reply_token,
        messages,
        notification_disabled,
    };
    api_request::line_post_request(
        body,
        format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE).as_str(),
    )
    .await?;
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyMessage {
    reply_token: String,
    messages: Vec<MessageObject>,
    notification_disabled: Option<bool>,
}
