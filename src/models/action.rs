use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Postback {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub data: String,
    pub display_text: Option<String>,
    /// This property will be abolished in the future.
    /// https://developers.line.biz/en/reference/messaging-api/#postback-action
    pub text: String,
}
