use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarouselTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub columns: Vec<Column>,
    pub image_aspect_ratio: Option<String>,
    pub image_size: Option<String>,
}

impl CarouselTemplate {
    pub fn new(columns: Vec<Column>) -> CarouselTemplate {
        CarouselTemplate {
            type_field: "carousel".to_string(),
            columns,
            image_aspect_ratio: None,
            image_size: None,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub thumbnail_image_url: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Option<Actions>,
    pub actions: Vec<Actions>,
}
