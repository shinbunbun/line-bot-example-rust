use serde::{Deserialize, Serialize};

use super::{quick_reply::QuickReply, sender::Sender, Message};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_content_url: String,
    pub preview_image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl Message<'_> for ImageMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImageMessage {
    pub fn new(original_content_url: String, preview_image_url: String) -> Self {
        ImageMessage {
            type_field: "image".to_string(),
            original_content_url,
            preview_image_url,
            quick_reply: None,
            sender: None,
        }
    }
}
