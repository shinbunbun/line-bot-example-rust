use serde::{Deserialize, Serialize};

use crate::{awc_wrapper::SendClientRequestFut, Client};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMembersCount {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMembersIds {
    pub member_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMember {
    pub display_name: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl Client {
    pub fn room_members_count(&self, room_id: &str) -> SendClientRequestFut<RoomMembersCount> {
        let url = format!(
            "{}/v2/bot/room/{}/members/count",
            API_ENDPOINT_BASE, room_id
        );
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn room_members_ids(
        &self,
        room_id: &str,
        start: Option<String>,
    ) -> SendClientRequestFut<RoomMembersIds> {
        let url = format!("{}/v2/bot/room/{}/members/ids", API_ENDPOINT_BASE, room_id);
        let mut params = vec![];
        if let Some(start) = start {
            params.push(("start", start));
        }
        SendClientRequestFut::new(self.get(&url, Some(&params), None, true))
    }

    pub fn room_member(&self, user_id: &str, room_id: &str) -> SendClientRequestFut<RoomMember> {
        let url = format!(
            "{}/v2/bot/room/{}/member/{}",
            API_ENDPOINT_BASE, room_id, user_id
        );
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn room_leave(&self, room_id: &str) -> SendClientRequestFut<()> {
        let url = format!("{}/v2/bot/room/{}/leave", API_ENDPOINT_BASE, room_id);
        SendClientRequestFut::new(self.post((), &url, None))
    }
}
