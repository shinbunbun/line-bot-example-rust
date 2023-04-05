use serde::{Deserialize, Serialize};

use crate::{awc_wrapper::SendClientRequestFut, models::empty::Empty, Client};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSummary {
    pub group_id: String,
    pub group_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembersCount {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembersIds {
    pub member_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub display_name: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
}

impl Client {
    pub fn group_summary(&self, group_id: &str) -> SendClientRequestFut<GroupSummary> {
        let url = format!("{}/v2/bot/group/{}/summary", API_ENDPOINT_BASE, group_id);
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn group_members_count(&self, group_id: &str) -> SendClientRequestFut<GroupSummary> {
        let url = format!(
            "{}/v2/bot/group/{}/members/count",
            API_ENDPOINT_BASE, group_id
        );
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn group_members_ids(
        &self,
        group_id: &str,
        start: Option<String>,
    ) -> SendClientRequestFut<GroupMembersIds> {
        let url = format!(
            "{}/v2/bot/group/{}/members/ids",
            API_ENDPOINT_BASE, group_id
        );
        let mut params = vec![];
        if let Some(start) = start {
            params.push(("start", start));
        }
        SendClientRequestFut::new(self.get(&url, Some(&params), None, true))
    }

    pub fn group_member(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> SendClientRequestFut<GroupMembersIds> {
        let url = format!(
            "{}/v2/bot/group/{}/member/{}",
            API_ENDPOINT_BASE, group_id, user_id
        );
        SendClientRequestFut::new(self.get(&url, None::<&[(); 0]>, None, true))
    }

    pub fn group_leave(&self, group_id: &str) -> SendClientRequestFut<Empty> {
        let url = format!("{}/v2/bot/group/{}/leave", API_ENDPOINT_BASE, group_id);
        SendClientRequestFut::new(self.post(Empty {}, &url, None))
    }
}
