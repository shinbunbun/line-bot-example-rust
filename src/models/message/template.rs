use serde::{Deserialize, Serialize};

use self::buttons::ButtonsTemplate;

pub mod buttons;
pub mod confirm;

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
    /* Confirm(ConfirmTemplate),
    Carousel(CarouselTemplate),
    ImageCarousel(ImageCarouselTemplate), */
}
