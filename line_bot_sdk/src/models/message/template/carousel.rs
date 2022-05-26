use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

use super::Template;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
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

/* impl CarouselTemplate {
    pub fn new(columns: Vec<Column>) -> CarouselTemplate {
        CarouselTemplate {
            type_field: "carousel".to_string(),
            columns,
            image_aspect_ratio: None,
            image_size: None,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_image_aspect_ratio(mut self, image_aspect_ratio: String) -> Self {
        self.image_aspect_ratio = Some(image_aspect_ratio);
        self
    }
    pub fn with_image_size(mut self, image_size: String) -> Self {
        self.image_size = Some(image_size);
        self
    }
} */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
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

/* impl Column {
    pub fn new(text: String, actions: Vec<Actions>) -> Column {
        Column {
            thumbnail_image_url: None,
            image_background_color: None,
            title: None,
            text,
            default_action: None,
            actions,
        }
    }
    pub fn with_thumbnail_image_url(mut self, thumbnail_image_url: String) -> Self {
        self.thumbnail_image_url = Some(thumbnail_image_url);
        self
    }
    pub fn with_image_background_color(mut self, image_background_color: String) -> Self {
        self.image_background_color = Some(image_background_color);
        self
    }
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    pub fn with_default_action(mut self, default_action: Actions) -> Self {
        self.default_action = Some(default_action);
        self
    }
} */
