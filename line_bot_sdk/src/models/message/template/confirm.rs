use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmTemplate {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    pub actions: Vec<Actions>,
}

impl ConfirmTemplate {
    pub fn new(text: String, actions: Vec<Actions>) -> Self {
        ConfirmTemplate {
            type_field: "confirm".to_string(),
            text,
            actions,
        }
    }
}
