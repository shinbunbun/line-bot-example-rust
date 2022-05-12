use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<Vec<Emoji>>,
}

impl TextMessage {
    pub fn new(text: String, emojis: Option<Vec<Emoji>>) -> Self {
        Self {
            type_field: "text".to_string(),
            text,
            emojis,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[serde(rename = "type")]
    pub index: u32,
    pub product_id: String,
    pub emoji_id: String,
}
