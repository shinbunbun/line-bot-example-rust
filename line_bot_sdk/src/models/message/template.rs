use serde::{Deserialize, Serialize};

use self::{
    buttons::ButtonsTemplate, carousel::CarouselTemplate, confirm::ConfirmTemplate,
    image_carousel::ImageCarouselTemplate,
};

use super::{quick_reply::QuickReply, sender::Sender, CommonFields};

pub mod buttons;
pub mod carousel;
pub mod confirm;
pub mod image_carousel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub alt_text: String,
    pub template: Template,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl CommonFields for TemplateMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Template {
    Buttons(ButtonsTemplate),
    Confirm(ConfirmTemplate),
    Carousel(CarouselTemplate),
    ImageCarousel(ImageCarouselTemplate),
}
