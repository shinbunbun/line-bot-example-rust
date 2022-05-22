use serde::{Deserialize, Serialize};

use super::{quick_reply::QuickReply, sender::Sender, Message};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StampMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub package_id: String,
    pub sticker_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl Message<'_> for StampMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl StampMessage {
    pub fn new(package_id: String, sticker_id: String) -> Self {
        StampMessage {
            type_field: "sticker".to_string(),
            package_id,
            sticker_id,
            quick_reply: None,
            sender: None,
        }
    }
}
