use crate::{
    awc_wrapper::SendClientRequestFut,
    models::{empty::Empty, message::MessageObject},
    Client,
};

use super::{ReplyMessage, API_ENDPOINT_BASE};

impl Client {
    pub fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = ReplyMessage {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(
            self.post(body, &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE)),
        )
    }
}
