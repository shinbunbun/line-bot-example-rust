use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageCarouselTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub image_url: String,
    pub action: Actions,
}
