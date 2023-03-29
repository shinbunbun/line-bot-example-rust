use crate::awc_wrapper::SendClientRequestByteFut;
use crate::client::API_ENDPOINT_BASE;

use crate::Client;

impl Client {
    pub async fn get_content(&self, message_id: &str) -> SendClientRequestByteFut {
        let url = format!(
            "{}/v2/bot/message/{}/content",
            API_ENDPOINT_BASE, message_id
        );
        SendClientRequestByteFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }
}
