use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

use super::Template;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CarouselTemplate {
    #[serde(rename = "type")]
    #[builder(default = "carousel".to_string())]
    pub type_field: String,
    pub columns: Vec<Column>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub image_aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub image_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<CarouselTemplate> for Template {
    fn from(template: CarouselTemplate) -> Self {
        Template::Carousel(template)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub thumbnail_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub image_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub title: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub default_action: Option<Actions>,
    pub actions: Vec<Actions>,
}
