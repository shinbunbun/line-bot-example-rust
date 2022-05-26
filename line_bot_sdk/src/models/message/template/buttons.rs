use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

use super::Template;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ButtonsTemplate {
    #[serde(rename = "type")]
    #[builder(default = "buttons".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub thumbnail_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub image_aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub image_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub image_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub title: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
    pub default_action: Actions,
    pub actions: Vec<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ButtonsTemplate> for Template {
    fn from(message: ButtonsTemplate) -> Self {
        Template::Buttons(message)
    }
}
