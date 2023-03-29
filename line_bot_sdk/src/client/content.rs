use crate::client::API_ENDPOINT_BASE;

use awc::SendClientRequest;

use crate::error::Error;
use crate::Client;

impl Client {
    pub async fn get_content(&self, message_id: &str) -> Result<SendClientRequest, Error> {
        let url = format!(
            "{}/v2/bot/message/{}/content",
            API_ENDPOINT_BASE, message_id
        );
        self.get(&url, None::<&[(); 0]>, None, true)
    }
}
