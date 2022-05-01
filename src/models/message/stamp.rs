use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StampMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub package_id: String,
    pub sticker_id: String,
}
