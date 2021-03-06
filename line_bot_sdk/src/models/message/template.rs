use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use self::{
    buttons::ButtonsTemplate, carousel::CarouselTemplate, confirm::ConfirmTemplate,
    image_carousel::ImageCarouselTemplate,
};

use super::{quick_reply::QuickReply, sender::Sender, MessageObject};

pub mod buttons;
pub mod carousel;
pub mod confirm;
pub mod image_carousel;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
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
        MessageObject::Template(Box::new(message))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Template {
    Buttons(Box<ButtonsTemplate>),
    Confirm(ConfirmTemplate),
    Carousel(CarouselTemplate),
    ImageCarousel(ImageCarouselTemplate),
}
