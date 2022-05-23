use serde::{Deserialize, Serialize};

use super::{quick_reply::QuickReply, sender::Sender, Message};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<Vec<Emoji>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl TextMessage {
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
}

impl Message<'_> for TextMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[serde(rename = "type")]
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}
