use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<Vec<Emoji>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[serde(rename = "type")]
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}
