use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationMessage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}
