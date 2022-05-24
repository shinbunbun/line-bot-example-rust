use crate::models::message::{
    audio::AudioMessage, image::ImageMessage, location::LocationMessage, stamp::StampMessage,
    text::TextMessage, video::VideoMessage,
};

use super::imagemap::ImagemapMessageBuilder;

pub struct MessageBuilder {}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {}
    }
    // pub fn audio(self) ->
    pub fn text_message(self, text: &str) -> TextMessage {
        TextMessage::new(text.to_string())
    }
    pub fn stamp_message(self, package_id: &str, sticker_id: &str) -> StampMessage {
        StampMessage::new(package_id.to_string(), sticker_id.to_string())
    }
    pub fn image_message(
        self,
        original_content_url: &str,
        preview_image_url: &str,
    ) -> ImageMessage {
        ImageMessage::new(
            original_content_url.to_string(),
            preview_image_url.to_string(),
        )
    }
    pub fn video_message(
        self,
        original_content_url: &str,
        preview_image_url: &str,
    ) -> VideoMessage {
        VideoMessage::new(
            original_content_url.to_string(),
            preview_image_url.to_string(),
        )
    }
    pub fn audio_message(self, original_content_url: &str, duration: u64) -> AudioMessage {
        AudioMessage::new(original_content_url.to_string(), duration)
    }
    pub fn location_message(
        self,
        title: &str,
        address: &str,
        latitude: f64,
        longitude: f64,
    ) -> LocationMessage {
        LocationMessage::new(title.to_string(), address.to_string(), latitude, longitude)
    }
    pub fn imagemap_message(self, base_url: &str, alt_text: &str) -> ImagemapMessageBuilder<()> {
        ImagemapMessageBuilder::new(base_url.to_string(), alt_text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::message::MessageBuilder;

    #[test]
    fn text_message() {
        let text_message = MessageBuilder::new().text_message("hello").build();
        println!("{:?}", text_message);
    }
}
