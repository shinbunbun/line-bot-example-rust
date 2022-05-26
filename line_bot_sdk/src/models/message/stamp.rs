use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, Message, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StampMessage {
    #[serde(rename = "type")]
    #[builder(default = "sticker".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub package_id: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub sticker_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<StampMessage> for MessageObject {
    fn from(message: StampMessage) -> Self {
        MessageObject::Stamp(message)
    }
}

/* impl Message<'_> for StampMessage {
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
    pub fn build(self) -> super::MessageObject {
        super::MessageObject::Stamp(self)
    }
} */
