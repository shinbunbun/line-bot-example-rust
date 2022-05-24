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
    /* pub fn build(self) -> quick_reply::QuickReply {
        quick_reply::QuickReply::new(self.items)
    } */
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
    /* pub fn build(self) -> quick_reply::Item {
        quick_reply::Item::new(self.image_url, self.action)
    } */
}

impl QuickReplyItemBuilder<Actions> {
    pub fn action(mut self, action: Actions) -> Self {
        self.action = action;
        self
    }
}
