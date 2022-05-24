use serde::{Deserialize, Serialize};
// use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actions {
    PostbackAction(PostbackAction),
    MessageAction(MessageAction),
    URIAction(URIAction),
    DatetimePickerAction(DatetimePickerAction),
    CameraAction(CameraAction),
    CameraRollAction(CameraRollAction),
    LocationAction(LocationAction),
    RichMenuSwitchAction(RichMenuSwitchAction),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct PostbackAction {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    pub data: String,
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,

    /// This property will be abolished in the future.
    /// https://developers.line.biz/en/reference/messaging-api/#postback-action
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_in_text: Option<String>,
}

impl PostbackAction {
    pub fn new(
        label: Option<String>,
        data: String,
        display_text: Option<String>,
        text: Option<String>,
        input_option: Option<String>,
        fill_in_text: Option<String>,
    ) -> Self {
        Self {
            type_field: "postback".to_string(),
            label,
            data,
            display_text,
            text,
            input_option,
            fill_in_text,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct MessageAction {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    pub text: String,
}

impl MessageAction {
    pub fn new(label: Option<String>, text: String) -> Self {
        Self {
            type_field: "message".to_string(),
            label,
            text,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct URIAction {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_uri: Option<AltUri>,
}

impl URIAction {
    pub fn new(label: Option<String>, uri: String, alt_uri: Option<AltUri>) -> Self {
        Self {
            type_field: "uri".to_string(),
            label,
            uri,
            alt_uri,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AltUri {
    pub desktop: String,
}

impl AltUri {
    pub fn new(desktop: String) -> Self {
        Self { desktop }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatetimePickerAction {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub data: String,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

impl DatetimePickerAction {
    pub fn new(
        data: String,
        mode: String,
        label: Option<String>,
        initial: Option<String>,
        max: Option<String>,
        min: Option<String>,
    ) -> Self {
        Self {
            type_field: "datetimepicker".to_string(),
            label,
            data,
            mode,
            initial,
            max,
            min,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}

impl CameraAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "camera".to_string(),
            label,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraRollAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}
impl CameraRollAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "cameraRoll".to_string(),
            label,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
}
impl LocationAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "location".to_string(),
            label,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuSwitchAction {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub rich_menu_alias_id: String,
    pub data: String,
}
