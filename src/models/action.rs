use serde::{Deserialize, Serialize};
// use validator::Validate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct PostbackAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    pub data: String,
    // #[validate(length(max = 300))]
    pub display_text: Option<String>,

    /// This property will be abolished in the future.
    /// https://developers.line.biz/en/reference/messaging-api/#postback-action
    // #[validate(length(max = 300))]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct MessageAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct URIAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub uri: String,
    pub alt_uri: Option<AltUri>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltUri {
    pub desktop: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatetimePickerAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub data: String,
    pub mode: String,
    pub initial: Option<String>,
    pub max: Option<String>,
    pub min: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraRollAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuSwitchAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}
