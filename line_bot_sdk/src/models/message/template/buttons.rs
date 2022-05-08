use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonsTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub thumbnail_image_url: Option<String>,
    pub image_aspect_ratio: Option<String>,
    pub image_size: Option<String>,
    pub image_background_color: Option<String>,
    pub title: Option<String>,
    pub text: String,
    pub default_action: Actions,
    pub actions: Vec<Actions>,
}
