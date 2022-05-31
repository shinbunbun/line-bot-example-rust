use serde::{Deserialize, Serialize};

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
