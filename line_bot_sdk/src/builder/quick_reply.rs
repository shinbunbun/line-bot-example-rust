use crate::models::{action::Actions, message::quick_reply};

pub struct QuickReplyBuilder {
    items: Vec<quick_reply::Item>,
}

impl QuickReplyBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add_item(mut self, item: quick_reply::Item) -> Self {
        self.items.push(item);
        self
    }
    pub fn build(self) -> quick_reply::QuickReply {
        quick_reply::QuickReply::new(self.items)
    }
}

pub struct QuickReplyItemBuilder<Actions> {
    pub image_url: Option<String>,
    pub action: Actions,
}

impl QuickReplyItemBuilder<()> {
    pub fn new() -> Self {
        Self {
            image_url: None,
            action: (),
        }
    }
    pub fn with_image_url(mut self, image_url: String) -> Self {
        self.image_url = Some(image_url);
        self
    }
    pub fn action(self, action: Actions) -> QuickReplyItemBuilder<Actions> {
        QuickReplyItemBuilder {
            image_url: self.image_url,
            action,
        }
    }
    /* pub fn build(self) -> quick_reply::Item {
        quick_reply::Item::new(self.image_url, self.action)
    } */
}

impl QuickReplyItemBuilder<Actions> {
    pub fn build(self) -> quick_reply::Item {
        quick_reply::Item::new(self.image_url, self.action)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{
            action::ActionBuilder,
            message::MessageBuilder,
            quick_reply::{QuickReplyBuilder, QuickReplyItemBuilder},
        },
        models::{
            action::{Actions, CameraAction, CameraRollAction, LocationAction},
            message::{
                quick_reply::Item, quick_reply::QuickReply, text::TextMessage, Message,
                MessageObject,
            },
        },
    };

    #[test]
    fn text_message() {
        let message = MessageBuilder::new()
            .text_message("text")
            .with_quick_reply(
                QuickReplyBuilder::new()
                    .add_item(
                        QuickReplyItemBuilder::new()
                            .action(ActionBuilder::new().camera_action("label").build())
                            .build(),
                    )
                    .add_item(
                        QuickReplyItemBuilder::new()
                            .action(ActionBuilder::new().camera_roll_action("label").build())
                            .build(),
                    )
                    .add_item(
                        QuickReplyItemBuilder::new()
                            .action(ActionBuilder::new().location_action("label").build())
                            .build(),
                    )
                    .build(),
            )
            .build();
        let message2 = MessageObject::Text(TextMessage {
            type_field: "text".to_string(),
            text: "text".to_string(),
            quick_reply: Some(QuickReply {
                items: vec![
                    Item {
                        type_field: "action".to_string(),
                        image_url: None,
                        action: Actions::CameraAction(CameraAction {
                            type_field: "camera".to_string(),
                            label: "label".to_string(),
                        }),
                    },
                    Item {
                        type_field: "action".to_string(),
                        image_url: None,
                        action: Actions::CameraRollAction(CameraRollAction {
                            type_field: "cameraRoll".to_string(),
                            label: "label".to_string(),
                        }),
                    },
                    Item {
                        type_field: "action".to_string(),
                        image_url: None,
                        action: Actions::LocationAction(LocationAction {
                            type_field: "location".to_string(),
                            label: "label".to_string(),
                        }),
                    },
                ],
            }),
            emojis: None,
            sender: None,
        });
        assert_eq!(message, message2);
    }
}
