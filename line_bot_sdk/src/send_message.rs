pub static API_ENDPOINT_BASE: &str = "https://api.line.me";

use serde::Serialize;

use crate::{client::Client, error::AppError, models::message::MessageObject};

impl Client {
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyMessage {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}
