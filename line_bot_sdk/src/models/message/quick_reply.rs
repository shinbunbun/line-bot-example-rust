use crate::models::action::Actions;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QuickReply {
    pub items: Vec<QuickReplyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QuickReplyItem {
    #[serde(rename = "type")]
    #[builder(default = "action".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub image_url: Option<String>,
    pub action: Actions,
}
