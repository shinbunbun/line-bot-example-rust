use serde::{Deserialize, Serialize};

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCarouselTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub columns: Vec<Column>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl ImageCarouselTemplate {
    pub fn new(columns: Vec<Column>) -> ImageCarouselTemplate {
        ImageCarouselTemplate {
            type_field: "image_carousel".to_string(),
            columns,
            quick_reply: None,
            sender: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub image_url: String,
    pub action: Actions,
}

impl Column {
    pub fn new(image_url: String, action: Actions) -> Column {
        Column { image_url, action }
    }
}
