use serde::{Deserialize, Serialize};

use crate::{
    awc_wrapper::SendClientRequestFut,
    models::{empty::Empty, message::MessageObject},
    Client,
};

use super::API_ENDPOINT_BASE;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplyRequest {
    reply_token: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushRequest {
    to: String,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MulticastRequest {
    to: Vec<String>,
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_aggregation_units: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BroadcastRequest {
    messages: Vec<MessageObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateReplyRequest {
    messages: Vec<MessageObject>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidatePushRequest {
    messages: Vec<MessageObject>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateMulticastRequest {
    messages: Vec<MessageObject>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaResponse {
    #[serde(rename = "type")]
    pub quota_type: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaConsumptionResponse {
    pub total_usage: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryReplyResponse {
    pub status: String,
    pub success: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryMulticastResponse {
    pub status: String,
    pub success: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryBroadcastResponse {
    pub status: String,
    pub success: Option<i64>,
}

impl Client {
    pub fn reply(
        &self,
        reply_token: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = ReplyRequest {
            reply_token: reply_token.to_string(),
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/reply", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn push(
        &self,
        x_line_retry_key: Option<&str>,
        to: &str,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
        custom_aggregation_units: Option<Vec<String>>,
    ) -> SendClientRequestFut<Empty> {
        let body = PushRequest {
            to: to.to_string(),
            messages,
            notification_disabled,
            custom_aggregation_units,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/push", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn multicast(
        &self,
        x_line_retry_key: Option<&str>,
        to: Vec<String>,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
        custom_aggregation_units: Option<Vec<String>>,
    ) -> SendClientRequestFut<Empty> {
        let body = MulticastRequest {
            to,
            messages,
            notification_disabled,
            custom_aggregation_units,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/multicast", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn broadcast(
        &self,
        x_line_retry_key: Option<&str>,
        messages: Vec<MessageObject>,
        notification_disabled: Option<bool>,
    ) -> SendClientRequestFut<Empty> {
        let body = BroadcastRequest {
            messages,
            notification_disabled,
        };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/broadcast", API_ENDPOINT_BASE),
            x_line_retry_key,
        ))
    }

    pub fn quota(&self) -> SendClientRequestFut<QuotaResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/message/quota", API_ENDPOINT_BASE),
            None::<&[(); 0]>,
            None,
            true,
        ))
    }

    pub fn quota_consumption(&self) -> SendClientRequestFut<QuotaConsumptionResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/message/quota/consumption", API_ENDPOINT_BASE),
            None::<&[(); 0]>,
            None,
            true,
        ))
    }

    pub fn delivery_reply(&self, date: &str) -> SendClientRequestFut<DeliveryReplyResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/message/delivery/reply", API_ENDPOINT_BASE),
            Some(&[("date", date)]),
            None,
            true,
        ))
    }

    pub fn delivery_multicast(
        &self,
        date: &str,
    ) -> SendClientRequestFut<DeliveryMulticastResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/message/delivery/multicast", API_ENDPOINT_BASE),
            Some(&[("date", date)]),
            None,
            true,
        ))
    }

    pub fn delivery_broadcast(
        &self,
        date: &str,
    ) -> SendClientRequestFut<DeliveryBroadcastResponse> {
        SendClientRequestFut::new(self.get(
            &format!("{}/v2/bot/message/delivery/broadcast", API_ENDPOINT_BASE),
            Some(&[("date", date)]),
            None,
            true,
        ))
    }

    pub fn validate_reply(&self, messages: Vec<MessageObject>) -> SendClientRequestFut<Empty> {
        let body = ValidateReplyRequest { messages };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/validate/reply", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn validate_push(&self, messages: Vec<MessageObject>) -> SendClientRequestFut<Empty> {
        let body = ValidatePushRequest { messages };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/validate/push", API_ENDPOINT_BASE),
            None,
        ))
    }

    pub fn validate_multicast(&self, messages: Vec<MessageObject>) -> SendClientRequestFut<Empty> {
        let body = ValidateMulticastRequest { messages };
        SendClientRequestFut::new(self.post(
            body,
            &format!("{}/v2/bot/message/validate/multicast", API_ENDPOINT_BASE),
            None,
        ))
    }
}
