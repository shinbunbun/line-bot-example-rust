use crate::error::AppError;

use self::{
    audio::AudioMessage, flex::FlexMessage, image::ImageMessage, imagemap::ImagemapMessage,
    location::LocationMessage, quick_reply::QuickReply, sender::Sender, stamp::StampMessage,
    template::TemplateMessage, text::TextMessage, video::VideoMessage,
};
use serde::{Deserialize, Serialize};

pub mod audio;
pub mod flex;
pub mod image;
pub mod imagemap;
pub mod location;
pub mod message_builder;
pub mod quick_reply;
pub mod sender;
pub mod stamp;
pub mod template;
pub mod text;
pub mod video;

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
/* pub struct MessageObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
    // #[serde(flatten)]
    // pub message: EachMessageFields,
} */
// pub enum EachMessageFields {
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageObject {
    Text(TextMessage),
    Stamp(StampMessage),
    Image(ImageMessage),
    Video(VideoMessage),
    Audio(AudioMessage),
    Location(LocationMessage),
    Imagemap(ImagemapMessage),
    Template(TemplateMessage),
    Flex(FlexMessage),
}

pub trait Message<'a> {
    fn from_json(json: &'a str) -> Result<Self, AppError>
    where
        Self: std::marker::Sized + Deserialize<'a>,
    {
        serde_json::from_str(json).map_err(AppError::SerdeJson)
    }
    fn with_quick_reply(self, quick_reply: QuickReply) -> Self;
    fn with_sender(self, sender: Sender) -> Self;
}
