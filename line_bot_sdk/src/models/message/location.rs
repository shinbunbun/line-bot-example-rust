use serde::{Deserialize, Serialize};

use super::{quick_reply::QuickReply, sender::Sender, Message};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<QuickReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Sender>,
}

impl Message<'_> for LocationMessage {
    fn with_quick_reply(mut self, quick_reply: super::quick_reply::QuickReply) -> Self {
        self.quick_reply = Some(quick_reply);
        self
    }
    fn with_sender(mut self, sender: super::sender::Sender) -> Self {
        self.sender = Some(sender);
        self
    }
}

impl LocationMessage {
    pub fn new(title: String, address: String, latitude: f64, longitude: f64) -> Self {
        LocationMessage {
            type_field: "location".to_string(),
            title,
            address,
            latitude,
            longitude,
            quick_reply: None,
            sender: None,
        }
    }
}
