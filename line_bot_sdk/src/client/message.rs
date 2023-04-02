use serde::Serialize;

use crate::{
    awc_wrapper::SendClientRequestFut,
    models::{empty::Empty, message::MessageObject},
    Client,
};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyRequest {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushRequest {
    to: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_aggregation_units: Option<Vec<String>>,
}

impl Client {
    pub fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = ReplyRequest {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn push(
        &self,
        x_line_retry_key: &str,
        to: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
        custom_aggregation_units: Option<Vec<String>>,
    ) -> SendClientRequestFut<Empty> {
        let body = PushRequest {
            to: to.to_string(),
            messages,
            notification_disabled,
            custom_aggregation_units,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/push", API_ENDPOINT_BASE),
            Some(x_line_retry_key),
        ))
    }
}
