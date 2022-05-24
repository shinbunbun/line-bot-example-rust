use crate::models::action::{
    Actions, AltUri, CameraAction, CameraRollAction, DatetimePickerAction, LocationAction,
    MessageAction, PostbackAction, RichmenuSwitchAction, URIAction,
};

pub struct ActionBuilder {}

impl ActionBuilder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn postback_action(self, data: &str) -> PostbackActionBuilder {
        PostbackActionBuilder::new(data)
    }
    pub fn message_action(self, text: &str) -> MessageActionBuilder {
        MessageActionBuilder::new(text)
    }
    pub fn uri_action(self, uri: &str) -> URIActionBuilder<()> {
        URIActionBuilder::new(uri)
    }
    pub fn datetime_picker_action(self, data: &str, mode: &str) -> DatetimePickerActionBuilder {
        DatetimePickerActionBuilder::new(data, mode)
    }
    pub fn camera_action(self, label: &str) -> CameraActionBuilder {
        CameraActionBuilder::new(label)
    }
    pub fn camera_roll_action(self, label: &str) -> CameraRollActionBuilder {
        CameraRollActionBuilder::new(label)
    }
    pub fn location_action(self, label: &str) -> LocationActionBuilder {
        LocationActionBuilder::new(label)
    }
    pub fn richmenu_switch_action(
        self,
        richmenu_alias_id: &str,
        data: &str,
    ) -> RichmenuSwitchActionBuilder {
        RichmenuSwitchActionBuilder::new(richmenu_alias_id, data)
    }
}

pub struct PostbackActionBuilder {
    data: String,
    label: Option<String>,
    display_text: Option<String>,
    text: Option<String>,
    input_option: Option<String>,
    fill_in_text: Option<String>,
}

impl PostbackActionBuilder {
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
            label: None,
            display_text: None,
            text: None,
            input_option: None,
            fill_in_text: None,
        }
    }
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    pub fn with_display_text(mut self, display_text: &str) -> Self {
        self.display_text = Some(display_text.to_string());
        self
    }
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }
    pub fn with_input_option(mut self, input_option: &str) -> Self {
        self.input_option = Some(input_option.to_string());
        self
    }
    pub fn with_fill_in_text(mut self, fill_in_text: &str) -> Self {
        self.fill_in_text = Some(fill_in_text.to_string());
        self
    }
    pub fn build(self) -> Actions {
        Actions::PostbackAction(PostbackAction::new(
            self.label,
            self.data,
            self.display_text,
            self.text,
            self.input_option,
            self.fill_in_text,
        ))
    }
}

pub struct MessageActionBuilder {
    label: Option<String>,
    text: String,
}

impl MessageActionBuilder {
    pub fn new(text: &str) -> Self {
        Self {
            label: None,
            text: text.to_string(),
        }
    }
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    pub fn build(self) -> Actions {
        Actions::MessageAction(MessageAction::new(self.label, self.text))
    }
}

pub struct URIActionBuilder<AltUri> {
    pub label: Option<String>,
    pub uri: String,
    pub alt_uri: Option<AltUri>,
}

impl URIActionBuilder<()> {
    pub fn new(uri: &str) -> Self {
        Self {
            label: None,
            uri: uri.to_string(),
            alt_uri: None,
        }
    }
    pub fn with_alt_uri(self, desktop: &str) -> URIActionBuilder<AltUri> {
        URIActionBuilder {
            label: self.label,
            uri: self.uri,
            alt_uri: Some(AltUri::new(desktop.to_string())),
        }
    }
}

impl<AltUri> URIActionBuilder<AltUri> {
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

impl URIActionBuilder<AltUri> {
    pub fn build(self) -> Actions {
        Actions::URIAction(URIAction::new(self.label, self.uri, self.alt_uri))
    }
}

pub struct DatetimePickerActionBuilder {
    data: String,
    mode: String,
    label: Option<String>,
    initial: Option<String>,
    max: Option<String>,
    min: Option<String>,
}

impl DatetimePickerActionBuilder {
    pub fn new(data: &str, mode: &str) -> Self {
        Self {
            data: data.to_string(),
            mode: mode.to_string(),
            label: None,
            initial: None,
            max: None,
            min: None,
        }
    }
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    pub fn with_initial(mut self, initial: &str) -> Self {
        self.initial = Some(initial.to_string());
        self
    }
    pub fn with_max(mut self, max: &str) -> Self {
        self.max = Some(max.to_string());
        self
    }
    pub fn with_min(mut self, min: &str) -> Self {
        self.min = Some(min.to_string());
        self
    }
    pub fn build(self) -> Actions {
        Actions::DatetimePickerAction(DatetimePickerAction::new(
            self.data,
            self.mode,
            self.label,
            self.initial,
            self.max,
            self.min,
        ))
    }
}

pub struct CameraActionBuilder {
    label: String,
}

impl CameraActionBuilder {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
    pub fn build(self) -> Actions {
        Actions::CameraAction(CameraAction::new(self.label))
    }
}

pub struct CameraRollActionBuilder {
    label: String,
}

impl CameraRollActionBuilder {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
    pub fn build(self) -> Actions {
        Actions::CameraRollAction(CameraRollAction::new(self.label))
    }
}

pub struct LocationActionBuilder {
    label: String,
}

impl LocationActionBuilder {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
    pub fn build(self) -> Actions {
        Actions::LocationAction(LocationAction::new(self.label))
    }
}

pub struct RichmenuSwitchActionBuilder {
    label: Option<String>,
    richmenu_alias_id: String,
    data: String,
}

impl RichmenuSwitchActionBuilder {
    pub fn new(richmenu_alias_id: &str, data: &str) -> Self {
        Self {
            label: None,
            richmenu_alias_id: richmenu_alias_id.to_string(),
            data: data.to_string(),
        }
    }
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    pub fn build(self) -> Actions {
        Actions::RichmenuSwitchAction(RichmenuSwitchAction::new(
            self.label,
            self.richmenu_alias_id,
            self.data,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::action::ActionBuilder,
        models::action::{
            Actions, AltUri, CameraAction, CameraRollAction, DatetimePickerAction, LocationAction,
            MessageAction, PostbackAction, RichmenuSwitchAction, URIAction,
        },
    };

    #[test]
    fn postback_action() {
        let action = ActionBuilder::new()
            .postback_action("data")
            .with_label("label")
            .with_display_text("display_text")
            .with_text("text")
            .with_fill_in_text("fill_in_text")
            .with_input_option("input_option")
            .build();
        let action2 = Actions::PostbackAction(PostbackAction {
            label: Some("label".to_string()),
            data: "data".to_string(),
            display_text: Some("display_text".to_string()),
            text: Some("text".to_string()),
            input_option: Some("input_option".to_string()),
            fill_in_text: Some("fill_in_text".to_string()),
            type_field: "postback".to_string(),
        });
        assert_eq!(action, action2);
    }

    #[test]
    fn message_action() {
        let action = ActionBuilder::new()
            .message_action("text")
            .with_label("label")
            .build();
        let action2 = Actions::MessageAction(MessageAction {
            label: Some("label".to_string()),
            text: "text".to_string(),
            type_field: "message".to_string(),
        });
        assert_eq!(action, action2);
    }

    #[test]
    fn uri_action() {
        let action = ActionBuilder::new()
            .uri_action("uri")
            .with_label("label")
            .with_alt_uri("desktop")
            .build();
        let action2 = Actions::URIAction(URIAction {
            label: Some("label".to_string()),
            uri: "uri".to_string(),
            alt_uri: Some(AltUri::new("desktop".to_string())),
            type_field: "uri".to_string(),
        });
        assert_eq!(action, action2);
    }

    #[test]
    fn datetime_picker_action() {
        let action = ActionBuilder::new()
            .datetime_picker_action("data", "datetime")
            .with_label("label")
            .with_initial("initial")
            .with_max("max")
            .with_min("min")
            .build();
        let action2 = Actions::DatetimePickerAction(DatetimePickerAction {
            label: Some("label".to_string()),
            data: "data".to_string(),
            mode: "datetime".to_string(),
            initial: Some("initial".to_string()),
            max: Some("max".to_string()),
            min: Some("min".to_string()),
            type_field: "datetimepicker".to_string(),
        });
        assert_eq!(action, action2);
    }

    #[test]
    fn camera_action() {
        let action = ActionBuilder::new().camera_action("label").build();
        let action2 = Actions::CameraAction(CameraAction::new("label".to_string()));
        assert_eq!(action, action2);
    }

    #[test]
    fn camera_roll_action() {
        let action = ActionBuilder::new().camera_roll_action("label").build();
        let action2 = Actions::CameraRollAction(CameraRollAction::new("label".to_string()));
        assert_eq!(action, action2);
    }

    #[test]
    fn location_action() {
        let action = ActionBuilder::new().location_action("label").build();
        let action2 = Actions::LocationAction(LocationAction::new("label".to_string()));
        assert_eq!(action, action2);
    }

    #[test]
    fn richmenu_action() {
        let action = ActionBuilder::new()
            .richmenu_switch_action("richmenu_alias_id", "data")
            .build();
        let action2 = Actions::RichmenuSwitchAction(RichmenuSwitchAction::new(
            None,
            "richmenu_alias_id".to_string(),
            "data".to_string(),
        ));
        assert_eq!(action, action2);
    }
}
