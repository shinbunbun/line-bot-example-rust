use crate::models::action::Actions;

use super::{
    audio::AudioMessage,
    image::ImageMessage,
    imagemap::{self, Area, ImagemapMessage},
    location::LocationMessage,
    quick_reply::{self, QuickReply},
    sender::Sender,
    stamp::StampMessage,
    text::TextMessage,
    video::VideoMessage,
    Message, MessageObject,
};

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

pub struct ImagemapMessageBuilder<BaseSize> {
    base_url: String,
    alt_text: String,
    base_size: BaseSize,
    actions: Vec<imagemap::Action>,
    quick_reply: Option<QuickReply>,
    sender: Option<Sender>,
}

impl<BaseSize> Message<'_> for ImagemapMessageBuilder<BaseSize> {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImagemapMessageBuilder<()> {
    pub fn new(base_url: String, alt_text: String) -> Self {
        Self {
            base_url,
            alt_text,
            base_size: (),
            actions: Vec::new(),
            quick_reply: None,
            sender: None,
        }
    }
}

impl<BaseSize> ImagemapMessageBuilder<BaseSize> {
    pub fn base_size(self, width: u64, height: u64) -> ImagemapMessageBuilder<imagemap::BaseSize> {
        ImagemapMessageBuilder {
            base_url: self.base_url,
            alt_text: self.alt_text,
            base_size: imagemap::BaseSize { width, height },
            actions: self.actions,
            quick_reply: self.quick_reply,
            sender: self.sender,
        }
    }
    /* pub fn add_uri_action(mut self, link_uri: &str, area: Area) -> Self {
        self.actions
            .push(imagemap::Action::URIAction(imagemap::URIAction::new(
                link_uri.to_string(),
                area,
            )));
        self
    } */
    pub fn add_action(mut self, action: imagemap::Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl ImagemapMessageBuilder<imagemap::BaseSize> {
    pub fn build(self) -> MessageObject {
        MessageObject::Imagemap(ImagemapMessage::new(
            self.base_url,
            self.alt_text,
            self.base_size,
            self.actions,
        ))
    }
}

pub struct ImageMapURIActionBuilder {
    link_uri: String,
    area: Area,
    label: Option<String>,
}

impl ImageMapURIActionBuilder {
    pub fn new(link_uri: &str, area: Area) -> Self {
        Self {
            link_uri: link_uri.to_string(),
            area,
            label: None,
        }
    }
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> imagemap::Action {
        imagemap::Action::URIAction(imagemap::URIAction::new(
            self.link_uri,
            self.area,
            self.label,
        ))
    }
}

pub struct ImageMapMessageActionBuilder {
    text: String,
    label: Option<String>,
    area: Area,
}

impl ImageMapMessageActionBuilder {
    pub fn new(text: &str, area: Area) -> Self {
        Self {
            text: text.to_string(),
            label: None,
            area,
        }
    }
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> imagemap::Action {
        imagemap::Action::MessageAction(imagemap::MessageAction::new(
            self.text, self.area, self.label,
        ))
    }
}

pub struct QuickReplyBuilder {
    items: Vec<quick_reply::Item>,
}

impl QuickReplyBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add_item(mut self, item: quick_reply::Item) -> Self {
        self.items.push(item);
        self
    }
    /* pub fn build(self) -> quick_reply::QuickReply {
        quick_reply::QuickReply::new(self.items)
    } */
}

pub struct QuickReplyItemBuilder<Actions> {
    pub image_url: Option<String>,
    pub action: Actions,
}

impl QuickReplyItemBuilder<()> {
    pub fn new() -> Self {
        Self {
            image_url: None,
            action: (),
        }
    }
    pub fn with_image_url(mut self, image_url: String) -> Self {
        self.image_url = Some(image_url);
        self
    }
    /* pub fn build(self) -> quick_reply::Item {
        quick_reply::Item::new(self.image_url, self.action)
    } */
}

impl QuickReplyItemBuilder<Actions> {
    pub fn action(mut self, action: Actions) -> Self {
        self.action = action;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::models::message::{
        imagemap::Area,
        message_builder::{ImageMapURIActionBuilder, MessageBuilder},
    };

    #[test]
    fn text_message() {
        let text_message = MessageBuilder::new().text_message("hello").build();
        println!("{:?}", text_message);
    }

    #[test]
    fn imagemap_message() {
        let imagemap_message = MessageBuilder::new()
            .imagemap_message(
                "https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun",
                "This is an imagemap",
            )
            .base_size(1040, 597)
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/intro/faculty/ubic/",
                    Area::new(26, 113, 525, 170),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://shinbunbun.info/about/",
                    Area::new(33, 331, 780, 177),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/",
                    Area::new(939, 484, 94, 105),
                )
                .build(),
            )
            .build();
        println!("{:?}", imagemap_message);
        // .area(0, 0, 10, 10);
    }
}
