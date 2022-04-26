use self::quick_reqly::QuickReply;
use serde::{Deserialize, Serialize};

mod quick_reqly;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub quick_reply: Option<QuickReply>,
}
