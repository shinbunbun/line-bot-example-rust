use serde::{Deserialize, Serialize};

use self::{
    buttons::ButtonsTemplate, carousel::CarouselTemplate, confirm::ConfirmTemplate,
    image_carousel::ImageCarouselTemplate,
};

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Template {
    Buttons(ButtonsTemplate),
    Confirm(ConfirmTemplate),
    Carousel(CarouselTemplate),
    ImageCarousel(ImageCarouselTemplate),
}
