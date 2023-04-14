use serde::{Deserialize, Serialize};

use crate::{awc_wrapper::SendClientRequestFut, Client};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLinkTokenResponse {
    pub link_token: String,
}

impl Client {
    pub fn user_link_token(&self, user_id: &str) -> SendClientRequestFut<UserLinkTokenResponse> {
        SendClientRequestFut::new(self.post(
            (),
            &format!("{}/v2/bot/user/{}/linkToken", API_ENDPOINT_BASE, user_id),
            None,
        ))
    }
}
