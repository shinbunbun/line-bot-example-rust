use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub display_name: String,
    pub user_id: String,
    pub language: String,
    pub picture_url: String,
    pub status_message: String,
}
