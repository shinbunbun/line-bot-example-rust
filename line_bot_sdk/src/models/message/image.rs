use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, MessageObject};

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessage {
    #[serde(rename = "type")]
    #[builder(default = "image".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub original_content_url: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub preview_image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ImageMessage> for MessageObject {
    fn from(message: ImageMessage) -> Self {
        MessageObject::Image(message)
    }
}
