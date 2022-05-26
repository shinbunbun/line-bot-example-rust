use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
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
    RichmenuSwitchAction(RichmenuSwitchAction),
}

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder /* , Validate */,
)]
#[serde(rename_all = "camelCase")]
pub struct PostbackAction {
    #[serde(rename = "type")]
    #[builder(default = "postback".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub display_text: Option<String>,

    /// This property will be abolished in the future.
    /// https://developers.line.biz/en/reference/messaging-api/#postback-action
    // #[validate(length(max = 300))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub input_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub fill_in_text: Option<String>,
}

impl From<PostbackAction> for Actions {
    fn from(action: PostbackAction) -> Self {
        Actions::PostbackAction(action)
    }
}

/* impl PostbackAction {
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
} */

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder /* , Validate */,
)]
#[serde(rename_all = "camelCase")]
pub struct MessageAction {
    #[serde(rename = "type")]
    #[builder(default = "message".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub label: Option<String>,
    // #[validate(length(max = 300))]
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub text: String,
}

/* impl MessageAction {
    pub fn new(label: Option<String>, text: String) -> Self {
        Self {
            type_field: "message".to_string(),
            label,
            text,
        }
    }
} */

impl From<MessageAction> for Actions {
    fn from(action: MessageAction) -> Self {
        Actions::MessageAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct URIAction {
    #[serde(rename = "type")]
    #[builder(default = "uri".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alt_uri: Option<AltUri>,
}

/* impl URIAction {
    pub fn new(label: Option<String>, uri: String, alt_uri: Option<AltUri>) -> Self {
        Self {
            type_field: "uri".to_string(),
            label,
            uri,
            alt_uri,
        }
    }
} */

impl From<URIAction> for Actions {
    fn from(action: URIAction) -> Self {
        Actions::URIAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AltUri {
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub desktop: String,
}

/* impl AltUri {
    pub fn new(desktop: String) -> Self {
        Self { desktop }
    }
} */

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DatetimePickerAction {
    #[serde(rename = "type")]
    #[builder(default = "datetimepicker".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub initial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub min: Option<String>,
}

/* impl DatetimePickerAction {
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
} */

impl From<DatetimePickerAction> for Actions {
    fn from(action: DatetimePickerAction) -> Self {
        Actions::DatetimePickerAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CameraAction {
    #[serde(rename = "type")]
    #[builder(default = "camera".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}

/* impl CameraAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "camera".to_string(),
            label,
        }
    }
} */

impl From<CameraAction> for Actions {
    fn from(action: CameraAction) -> Self {
        Actions::CameraAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CameraRollAction {
    #[serde(rename = "type")]
    #[builder(default = "cameraRoll".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}
/* impl CameraRollAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "cameraRoll".to_string(),
            label,
        }
    }
} */

impl From<CameraRollAction> for Actions {
    fn from(action: CameraRollAction) -> Self {
        Actions::CameraRollAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LocationAction {
    #[serde(rename = "type")]
    #[builder(default = "location".to_string())]
    pub type_field: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub label: String,
}
/* impl LocationAction {
    pub fn new(label: String) -> Self {
        Self {
            type_field: "location".to_string(),
            label,
        }
    }
} */

impl From<LocationAction> for Actions {
    fn from(action: LocationAction) -> Self {
        Actions::LocationAction(action)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RichmenuSwitchAction {
    #[serde(rename = "type")]
    #[builder(default = "richMenuSwitch".to_string())]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(transform = |x: Option<String>| x.map(|x| x.to_string())))]
    pub label: Option<String>,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub richmenu_alias_id: String,
    #[builder(setter(transform = |x: &str| x.to_string()))]
    pub data: String,
}

/* impl RichmenuSwitchAction {
    pub fn new(label: Option<String>, richmenu_alias_id: String, data: String) -> Self {
        Self {
            type_field: "richMenuSwitch".to_string(),
            label,
            richmenu_alias_id,
            data,
        }
    }
} */

impl From<RichmenuSwitchAction> for Actions {
    fn from(action: RichmenuSwitchAction) -> Self {
        Actions::RichmenuSwitchAction(action)
    }
}
