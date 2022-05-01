use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagemapMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub base_url: String,
    pub alt_text: String,
    pub base_size: BaseSize,
    pub video: Option<Video>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSize {
    width: u64,
    height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    original_content_url: String,
    preview_image_url: String,
    area: Area,
    external_link: Option<ExternalLink>,
    actions: Actions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLink {
    link_uri: String,
    label: String,
}
