use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

use super::Template;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmTemplate {
    #[serde(rename = "type")]
    #[builder(default = "confirm".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
    pub actions: Vec<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sender: Option<Sender>,
}

impl From<ConfirmTemplate> for Template {
    fn from(message: ConfirmTemplate) -> Self {
        Template::Confirm(message)
    }
}
