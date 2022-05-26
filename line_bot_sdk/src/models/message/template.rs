use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use self::{
    buttons::ButtonsTemplate, carousel::CarouselTemplate, confirm::ConfirmTemplate,
    image_carousel::ImageCarouselTemplate,
};

use super::{quick_reply::QuickReply, sender::Sender, Message, MessageObject};

pub mod buttons;
pub mod carousel;
pub mod confirm;
pub mod image_carousel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMessage {
    #[serde(rename = "type")]
    #[builder(default = "template".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub alt_text: String,
    pub template: Template,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<TemplateMessage> for MessageObject {
    fn from(message: TemplateMessage) -> Self {
        MessageObject::Template(message)
    }
}

/* impl Message<'_> for TemplateMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl TemplateMessage {
    pub fn new(alt_text: String, template: Template) -> Self {
        TemplateMessage {
            type_field: "template".to_string(),
            alt_text,
            template,
            quick_reply: None,
            sender: None,
        }
    }
} */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Template {
    Buttons(ButtonsTemplate),
    Confirm(ConfirmTemplate),
    Carousel(CarouselTemplate),
    ImageCarousel(ImageCarouselTemplate),
}
