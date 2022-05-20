use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCarouselTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub columns: Vec<Column>,
}

impl ImageCarouselTemplate {
    pub fn new(columns: Vec<Column>) -> ImageCarouselTemplate {
        ImageCarouselTemplate {
            type_field: "image_carousel".to_string(),
            columns,
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
