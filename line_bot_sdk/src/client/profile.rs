use crate::client::SendClientRequestFut;
use crate::client::API_ENDPOINT_BASE;
use crate::models::profile::Profile;
use crate::Client;

impl Client {
    pub fn get_profile(&self, user_id: &str) -> SendClientRequestFut<Profile> {
        let url = format!("{}/v2/bot/profile/{}", API_ENDPOINT_BASE, user_id);
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }
}
