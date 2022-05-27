use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

use super::Template;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ImageCarouselTemplate {
    #[serde(rename = "type")]
    #[builder(default = "image_carousel".to_string())]
    pub type_field: String,
    pub columns: Vec<Column>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ImageCarouselTemplate> for Template {
    fn from(template: ImageCarouselTemplate) -> Self {
        Template::ImageCarousel(template)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub image_url: String,
    pub action: Actions,
}
