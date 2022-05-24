use crate::models::action::Actions;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickReply {
    pub items: Vec<Item>,
}

impl QuickReply {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub action: Actions,
}

impl Item {
    pub fn new(image_url: Option<String>, action: Actions) -> Self {
        Self {
            type_field: "action".to_string(),
            image_url,
            action,
        }
    }
}
