use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VideoMessage {
    #[serde(rename = "type")]
    #[builder(default = "video".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub original_content_url: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub preview_image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub tracking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<VideoMessage> for MessageObject {
    fn from(message: VideoMessage) -> Self {
        MessageObject::Video(message)
    }
}

/* impl Message<'_> for VideoMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl VideoMessage {
    pub fn new(original_content_url: String, preview_image_url: String) -> Self {
        VideoMessage {
            type_field: "video".to_string(),
            original_content_url,
            preview_image_url,
            tracking_id: None,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_tracking_id(mut self, tracking_id: String) -> Self {
        self.tracking_id = Some(tracking_id);
        self
    }
    pub fn build(self) -> super::MessageObject {
        super::MessageObject::Video(self)
    }
} */
