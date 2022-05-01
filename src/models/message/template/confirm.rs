use serde::{Deserialize, Serialize};

use crate::models::action::Actions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Confirm {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub actions: Actions,
}
