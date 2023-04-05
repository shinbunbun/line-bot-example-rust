use serde::Deserialize;
use serde::Serialize;

use crate::client::SendClientRequestFut;
use crate::client::API_ENDPOINT_BASE;
use crate::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub display_name: String,
    pub user_id: String,
    pub language: String,
    pub picture_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub user_ids: Vec<String>,
    pub next: Option<String>,
}

impl Client {
    pub fn profile(&self, user_id: &str) -> SendClientRequestFut<Profile> {
        let url = format!("{}/v2/bot/profile/{}", API_ENDPOINT_BASE, user_id);
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn followers_ids(
        &self,
        limit: Option<&str>,
        start: Option<&str>,
    ) -> SendClientRequestFut<Followers> {
        let url = format!("{}/v2/bot/followers/ids", API_ENDPOINT_BASE);
        let mut params = vec![];
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(start) = start {
            params.push(("start", start));
        }
        SendClientRequestFut::new(self.get(&url, Some(&params), None, true))
    }
}
