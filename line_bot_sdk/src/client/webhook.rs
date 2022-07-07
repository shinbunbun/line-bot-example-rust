use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::Client;

use super::API_ENDPOINT_BASE;

impl Client {
    pub async fn set_webhook_event_url(&self, endpoint: &str) -> Result<(), Error> {
        self.put(
            WebhookEndpointStruct {
                endpoint: endpoint.to_string(),
            },
            &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
        )
        .await?;
        Ok(())
    }

    pub async fn get_webhook_endpoint_info(&self) -> Result<WebhookEndpointInfo, Error> {
        let mut res = self
            .get(
                &format!("{}/v2/bot/channel/webhook/endpoint", API_ENDPOINT_BASE),
                None::<&[(); 0]>,
            )
            .await?;
        let res_body = res
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
    }

    pub async fn test_webhook_endpoint_url(
        &self,
        endpoint: Option<String>,
    ) -> Result<TestWebhookEndpointUrlResponse, Error> {
        let mut res = match endpoint {
            Some(endpoint) => {
                self.post(
                    WebhookEndpointStruct {
                        endpoint: endpoint.to_string(),
                    },
                    &format!("{}/v2/bot/channel/webhook/endpoint/test", API_ENDPOINT_BASE),
                )
                .await?
            }
            None => {
                self.post(
                    (),
                    &format!("{}/v2/bot/channel/webhook/endpoint/test", API_ENDPOINT_BASE),
                )
                .await?
            }
        };
        let res_body = res
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebhookEndpointStruct {
    endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpointInfo {
    pub endpoint: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestWebhookEndpointUrlResponse {
    pub success: bool,
    pub timestamp: u64,
    pub status_code: i16,
    pub reason: String,
    pub detail: String,
}
