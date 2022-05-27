use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
// use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Actions {
    PostbackAction(PostbackAction),
    MessageAction(MessageAction),
    URIAction(URIAction),
    DatetimePickerAction(DatetimePickerAction),
    CameraAction(CameraAction),
    CameraRollAction(CameraRollAction),
    LocationAction(LocationAction),
    RichmenuSwitchAction(RichmenuSwitchAction),
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct PostbackAction {
    #[serde(rename = "type")]
    #[builder(default = "postback".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub display_text: Option<String>,

    /// This property will be abolished in the future.
    /// https://developers.line.biz/en/reference/messaging-api/#postback-action
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub input_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub fill_in_text: Option<String>,
}

impl From<PostbackAction> for Actions {
    fn from(action: PostbackAction) -> Self {
        Actions::PostbackAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder /* , Validate */)]
#[serde(rename_all = "camelCase")]
pub struct MessageAction {
    #[serde(rename = "type")]
    #[builder(default = "message".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
}

impl From<MessageAction> for Actions {
    fn from(action: MessageAction) -> Self {
        Actions::MessageAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct URIAction {
    #[serde(rename = "type")]
    #[builder(default = "uri".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alt_uri: Option<AltUri>,
}

impl From<URIAction> for Actions {
    fn from(action: URIAction) -> Self {
        Actions::URIAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AltUri {
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub desktop: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DatetimePickerAction {
    #[serde(rename = "type")]
    #[builder(default = "datetimepicker".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub initial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub min: Option<String>,
}

impl From<DatetimePickerAction> for Actions {
    fn from(action: DatetimePickerAction) -> Self {
        Actions::DatetimePickerAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CameraAction {
    #[serde(rename = "type")]
    #[builder(default = "camera".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}

impl From<CameraAction> for Actions {
    fn from(action: CameraAction) -> Self {
        Actions::CameraAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CameraRollAction {
    #[serde(rename = "type")]
    #[builder(default = "cameraRoll".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}

impl From<CameraRollAction> for Actions {
    fn from(action: CameraRollAction) -> Self {
        Actions::CameraRollAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LocationAction {
    #[serde(rename = "type")]
    #[builder(default = "location".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}

impl From<LocationAction> for Actions {
    fn from(action: LocationAction) -> Self {
        Actions::LocationAction(action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichmenuSwitchAction {
    #[serde(rename = "type")]
    #[builder(default = "richMenuSwitch".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: &str| Some(x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub richmenu_alias_id: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
}

impl From<RichmenuSwitchAction> for Actions {
    fn from(action: RichmenuSwitchAction) -> Self {
        Actions::RichmenuSwitchAction(action)
    }
}
