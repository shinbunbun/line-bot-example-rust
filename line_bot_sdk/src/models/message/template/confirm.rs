use serde::{Deserialize, Serialize};

use crate::models::{
    action::Actions,
    message::{quick_reply::QuickReply, sender::Sender},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    pub actions: Vec<Actions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl ConfirmTemplate {
    pub fn new(text: String, actions: Vec<Actions>) -> Self {
        ConfirmTemplate {
            type_field: "confirm".to_string(),
            text,
            actions,
            quick_reply: None,
            sender: None,
        }
    }
}
