use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub original_content_url: String,
    pub duration: u64,
}
