use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, MessageObject};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub index: u32,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub product_id: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub emoji_id: String,
}
