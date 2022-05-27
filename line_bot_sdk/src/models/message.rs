use self::{
    audio::AudioMessage, flex::FlexMessage, image::ImageMessage, imagemap::ImagemapMessage,
    location::LocationMessage, stamp::StampMessage, template::TemplateMessage, text::TextMessage,
    video::VideoMessage,
};
use crate::error::Error;
use serde::{Deserialize, Serialize};

pub mod audio;
pub mod flex;
pub mod image;
pub mod imagemap;
pub mod location;
pub mod quick_reply;
pub mod sender;
pub mod stamp;
pub mod template;
pub mod text;
pub mod video;

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

impl MessageObject {
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json).map_err(Error::SerdeJsonError)
    }
}
