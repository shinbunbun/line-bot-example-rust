use serde::{Deserialize, Serialize};

use crate::{awc_wrapper::SendClientRequestFut, Client};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
    pub user_id: String,
    pub basic_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_id: Option<String>,
    pub display_name: String,
    pub picture_url: Option<String>,
    pub chat_mode: String,
    pub mark_as_read_mode: String,
}

impl Client {
    pub fn bot_info(&self) -> SendClientRequestFut<BotInfo> {
        let url = format!("{}/v2/bot/info", API_ENDPOINT_BASE);
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }
}
