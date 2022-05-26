use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, Message, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    #[serde(rename = "type")]
    #[builder(default = "text".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub emojis: Option<Vec<Emoji>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<TextMessage> for MessageObject {
    fn from(message: TextMessage) -> Self {
        MessageObject::Text(message)
    }
}

/* impl TextMessage {
    pub fn new(text: String) -> Self {
        Self {
            type_field: "text".to_string(),
            text,
            emojis: None,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_emojis(mut self, emojis: Vec<Emoji>) -> Self {
        self.emojis = Some(emojis);
        self
    }
    pub fn build(self) -> super::MessageObject {
        super::MessageObject::Text(self)
    }
} */

/* impl Message<'_> for TextMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
} */

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub index: u32,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub product_id: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub emoji_id: String,
}
