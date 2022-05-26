use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, Message, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AudioMessage {
    #[serde(rename = "type")]
    #[builder(default = "audio".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub original_content_url: String,
    pub duration: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<AudioMessage> for MessageObject {
    fn from(message: AudioMessage) -> Self {
        MessageObject::Audio(message)
    }
}

/*
impl Message<'_> for AudioMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl AudioMessage {
    pub fn new(original_content_url: String, duration: u64) -> Self {
        AudioMessage {
            type_field: "audio".to_string(),
            original_content_url,
            duration,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn build(self) -> super::MessageObject {
        super::MessageObject::Audio(self)
    }
}
 */
