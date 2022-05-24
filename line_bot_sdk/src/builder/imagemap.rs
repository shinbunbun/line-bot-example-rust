use crate::models::message::{
    imagemap::{self, Area, ImagemapMessage},
    quick_reply::QuickReply,
    sender::Sender,
    Message, MessageObject,
};

pub struct ImagemapMessageBuilder<BaseSize> {
    base_url: String,
    alt_text: String,
    base_size: BaseSize,
    actions: Vec<imagemap::Action>,
    quick_reply: Option<QuickReply>,
    sender: Option<Sender>,
}

impl<BaseSize> Message<'_> for ImagemapMessageBuilder<BaseSize> {
    fn with_quick_reply(mut self, quick_reply: QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl ImagemapMessageBuilder<()> {
    pub fn new(base_url: String, alt_text: String) -> Self {
        Self {
            base_url,
            alt_text,
            base_size: (),
            actions: Vec::new(),
            quick_reply: None,
            sender: None,
        }
    }
}

impl<BaseSize> ImagemapMessageBuilder<BaseSize> {
    pub fn base_size(self, width: u64, height: u64) -> ImagemapMessageBuilder<imagemap::BaseSize> {
        ImagemapMessageBuilder {
            base_url: self.base_url,
            alt_text: self.alt_text,
            base_size: imagemap::BaseSize { width, height },
            actions: self.actions,
            quick_reply: self.quick_reply,
            sender: self.sender,
        }
    }
    /* pub fn add_uri_action(mut self, link_uri: &str, area: Area) -> Self {
        self.actions
            .push(imagemap::Action::URIAction(imagemap::URIAction::new(
                link_uri.to_string(),
                area,
            )));
        self
    } */
    pub fn add_action(mut self, action: imagemap::Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl ImagemapMessageBuilder<imagemap::BaseSize> {
    pub fn build(self) -> MessageObject {
        MessageObject::Imagemap(ImagemapMessage::new(
            self.base_url,
            self.alt_text,
            self.base_size,
            self.actions,
        ))
    }
}

pub struct ImageMapURIActionBuilder {
    link_uri: String,
    area: Area,
    label: Option<String>,
}

impl ImageMapURIActionBuilder {
    pub fn new(link_uri: &str, area: Area) -> Self {
        Self {
            link_uri: link_uri.to_string(),
            area,
            label: None,
        }
    }
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> imagemap::Action {
        imagemap::Action::URIAction(imagemap::URIAction::new(
            self.link_uri,
            self.area,
            self.label,
        ))
    }
}

pub struct ImageMapMessageActionBuilder {
    text: String,
    label: Option<String>,
    area: Area,
}

impl ImageMapMessageActionBuilder {
    pub fn new(text: &str, area: Area) -> Self {
        Self {
            text: text.to_string(),
            label: None,
            area,
        }
    }
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> imagemap::Action {
        imagemap::Action::MessageAction(imagemap::MessageAction::new(
            self.text, self.area, self.label,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{imagemap::ImageMapURIActionBuilder, message::MessageBuilder},
        models::message::imagemap::Area,
    };

    #[test]
    fn imagemap_message() {
        let imagemap_message = MessageBuilder::new()
            .imagemap_message(
                "https://youkan-storage.s3.ap-northeast-1.amazonaws.com/ubic_bunbun",
                "This is an imagemap",
            )
            .base_size(1040, 597)
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/intro/faculty/ubic/",
                    Area::new(26, 113, 525, 170),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://shinbunbun.info/about/",
                    Area::new(33, 331, 780, 177),
                )
                .build(),
            )
            .add_action(
                ImageMapURIActionBuilder::new(
                    "https://www.u-aizu.ac.jp/",
                    Area::new(939, 484, 94, 105),
                )
                .build(),
            )
            .build();
        println!("{:?}", imagemap_message);
        // .area(0, 0, 10, 10);
    }
}
