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
                None,
                true,
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
        endpoint: Option<&str>,
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
                    Empty {},
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Empty {}

#[cfg(test)]
mod test {
    use std::env;

    use chrono::Local;

    use crate::Client;

    fn get_client() -> crate::Client {
        crate::Client::new(
            env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
            env::var("CHANNEL_SECRET").unwrap(),
            env::var("CHANNEL_ID").unwrap(),
        )
    }

    #[actix_web::test]
    async fn test_webhook_endpoint_url() {
        let client = get_client();
        let endpoint = &format!("https://example.com/{}", Local::now().timestamp());
        test_set_webhook_event_url(&client, endpoint).await;
        test_get_webhook_endpoint_info(&client, endpoint).await;
        // test_test_webhook_endpoint_url(&client).await;
        // test_test_webhook_endpoint_url_with_endpoint(&client, endpoint).await;
    }

    async fn test_set_webhook_event_url(client: &Client, endpoint: &str) {
        client.set_webhook_event_url(endpoint).await.unwrap();
    }

    async fn test_get_webhook_endpoint_info(client: &Client, endpoint: &str) {
        let info = client.get_webhook_endpoint_info().await.unwrap();
        assert_eq!(info.endpoint, endpoint);
    }

    /* async fn test_test_webhook_endpoint_url(client: &Client) {
        client.test_webhook_endpoint_url(None).await.unwrap();
    }

    async fn test_test_webhook_endpoint_url_with_endpoint(client: &Client, endpoint: &str) {
        client
            .test_webhook_endpoint_url(Some(endpoint))
            .await
            .unwrap();
    } */
}
