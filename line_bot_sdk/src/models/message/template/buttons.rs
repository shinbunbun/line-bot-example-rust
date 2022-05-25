use serde::{Deserialize, Serialize};

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonsTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub text: String,
    pub default_action: Actions,
    pub actions: Vec<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl ButtonsTemplate {
    pub fn new(text: String, default_action: Actions, actions: Vec<Actions>) -> Self {
        ButtonsTemplate {
            type_field: "buttons".to_string(),
            thumbnail_image_url: None,
            image_aspect_ratio: None,
            image_size: None,
            image_background_color: None,
            title: None,
            text,
            default_action,
            actions,
            quick_reply: None,
            sender: None,
        }
    }
    pub fn with_thumbnail_image_url(mut self, thumbnail_image_url: String) -> Self {
        self.thumbnail_image_url = Some(thumbnail_image_url);
        self
    }
    pub fn with_image_aspect_ratio(mut self, image_aspect_ratio: String) -> Self {
        self.image_aspect_ratio = Some(image_aspect_ratio);
        self
    }
    pub fn with_image_size(mut self, image_size: String) -> Self {
        self.image_size = Some(image_size);
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
}
