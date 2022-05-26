use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{quick_reply::QuickReply, sender::Sender, MessageObject};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapMessage {
    #[serde(rename = "type")]
    #[builder(default = "imagemap".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub base_url: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub alt_text: String,
    #[builder(setter(transform = |width: u64, height: u64| BaseSize {width, height}))]
    pub base_size: BaseSize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub video: Option<Video>,
    pub actions: Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ImagemapMessage> for MessageObject {
    fn from(message: ImagemapMessage) -> Self {
        MessageObject::Imagemap(message)
    }
}

/* impl Message<'_> for ImagemapMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImagemapMessage {
    pub fn new(
        base_url: String,
        alt_text: String,
        base_size: BaseSize,
        actions: Vec<Action>,
    ) -> Self {
        ImagemapMessage {
            type_field: "imagemap".to_string(),
            base_url,
            alt_text,
            base_size,
            video: None,
            actions,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_video(mut self, video: Video) -> Self {
        self.video = Some(video);
        self
    }
} */

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSize {
    pub width: u64,
    pub height: u64,
}

/* impl BaseSize {
    pub fn new(width: u64, height: u64) -> Self {
        BaseSize { width, height }
    }
} */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[builder(setter(transform = |x: &str| x.to_string()))]
    original_content_url: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    preview_image_url: String,
    #[builder(setter(transform = |x: u64, y: u64, width: u64, height: u64| Area {x, y, width, height}))]
    area: Area,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    external_link: Option<ExternalLink>,
    actions: Vec<Action>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

/* impl Area {
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> Self {
        Area {
            x,
            y,
            width,
            height,
        }
    }
} */

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLink {
    #[builder(setter(transform = |x: &str| x.to_string()))]
    link_uri: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    URIAction(ImagemapURIAction),
    MessageAction(ImagemapMessageAction),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapURIAction {
    #[serde(rename = "type")]
    #[builder(default = "uri".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub link_uri: String,
    #[builder(setter(transform = |x: u64, y: u64, width: u64, height: u64| Area {x, y, width, height}))]
    pub area: Area,
}

impl From<ImagemapURIAction> for Action {
    fn from(action: ImagemapURIAction) -> Self {
        Action::URIAction(action)
    }
}

/* impl URIAction {
    pub fn new(link_uri: String, area: Area, label: Option<String>) -> Self {
        URIAction {
            type_field: "uri".to_string(),
            label,
            link_uri,
            area,
        }
    }
} */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapMessageAction {
    #[serde(rename = "type")]
    #[builder(default = "message".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
    #[builder(setter(transform = |x: u64, y: u64, width: u64, height: u64| Area {x, y, width, height}))]
    pub area: Area,
}

impl From<ImagemapMessageAction> for Action {
    fn from(action: ImagemapMessageAction) -> Self {
        Action::MessageAction(action)
    }
}

/* impl MessageAction {
    pub fn new(text: String, area: Area, label: Option<String>) -> Self {
        MessageAction {
            type_field: "message".to_string(),
            label,
            text,
            area,
        }
    }
} */
