use serde::Deserialize;
use serde::Serialize;

use crate::awc_wrapper::SendClientRequestFut;
use crate::models::empty::Empty;
use crate::Client;
use crate::Error;

use super::API_ENDPOINT_BASE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    pub client_id: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenV2Response {
    pub client_id: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokensKidResponse {
    pub kids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTokenV2Response {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

impl Client {
    pub fn issue_token(&self, client_assertion: &str) -> SendClientRequestFut<IssueTokenResponse> {
        SendClientRequestFut::new(self.post_form(
            &[
                ("grant_type", "client_credentials"),
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_assertion", client_assertion),
            ],
            &format!("{}/oauth2/v2.1/token", API_ENDPOINT_BASE),
        ))
    }

    pub async fn verify_token(&self, access_token: &str) -> Result<VerifyTokenResponse, Error> {
        let res_body = self
            .get(
                &format!("{}/oauth2/v2.1/verify", API_ENDPOINT_BASE),
                Some(&[("access_token", access_token)]),
                Some("application/x-www-form-urlencoded"),
                false,
            )?
            .await
            .map_err(Error::AwcSendRequestError)?
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
    }

    pub async fn get_tokens_kid(
        &self,
        client_assertion: &str,
    ) -> Result<GetTokensKidResponse, Error> {
        let res_body = self
            .get(
                &format!("{}/oauth2/v2.1/tokens/kid", API_ENDPOINT_BASE),
                Some(&[
                    (
                        "client_assertion_type",
                        "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                    ),
                    ("client_assertion", client_assertion),
                ]),
                Some("application/x-www-form-urlencoded"),
                false,
            )?
            .await
            .map_err(Error::AwcSendRequestError)?
            .body()
            .await
            .map_err(Error::ActixWebPayloadError)?
            .to_vec();
        serde_json::from_slice(&res_body).map_err(Error::SerdeJsonError)
    }

    pub fn revoke_token(
        &self,
        client_id: &str,
        client_secret: &str,
        access_token: &str,
    ) -> SendClientRequestFut<Empty> {
        SendClientRequestFut::new(self.post_form(
            &[
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("access_token", access_token),
            ],
            &format!("{}/oauth2/v2.1/revoke", API_ENDPOINT_BASE),
        ))
    }

    pub fn issue_token_v2(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> SendClientRequestFut<IssueTokenV2Response> {
        SendClientRequestFut::new(self.post_form(
            &[
                ("grant_type", "client_credentials"),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ],
            &format!("{}/v2/oauth/accessToken", API_ENDPOINT_BASE),
        ))
    }

    pub fn verify_token_v2(
        &self,
        access_token: &str,
    ) -> SendClientRequestFut<VerifyTokenV2Response> {
        SendClientRequestFut::new(self.post_form(
            &[("access_token", access_token)],
            &format!("{}/v2/oauth/verify", API_ENDPOINT_BASE),
        ))
    }

    pub fn revoke_token_v2(&self, access_token: &str) -> SendClientRequestFut<Empty> {
        SendClientRequestFut::new(self.post_form(
            &[("access_token", access_token)],
            &format!("{}/v2/oauth/revoke", API_ENDPOINT_BASE),
        ))
    }
}

/* #[cfg(test)]
mod test {
    use std::env;

    use crate::{jwt, Client};

    fn create_client() -> Client {
        let channel_access_token = env::var("CHANNEL_ACCESS_TOKEN").unwrap();
        let channel_secret = env::var("CHANNEL_SECRET").unwrap();
        let channel_id = env::var("CHANNEL_ID").unwrap();

        Client::new(channel_access_token, channel_secret, channel_id)
    }

    async fn test_verify_token(client: &Client, access_token: &str) {
        let verify_token_response = client.verify_token(access_token).await.unwrap();
        assert_eq!(verify_token_response.client_id, client.get_channel_id());
    }

    async fn test_verify_token_error(client: &Client, access_token: &str) {
        let verify_token_response = client.verify_token(access_token).await;
        assert!(verify_token_response.is_err());
    }

    async fn test_get_tokens_kid(
        kid: &str,
        private_key: &str,
        client: &Client,
        token_key_id: &str,
    ) {
        let jwt = jwt::create_jwt(kid, client.get_channel_id(), private_key).unwrap();

        let mut get_tokens_kid_response = client.get_tokens_kid(&jwt).await.unwrap();
        get_tokens_kid_response.kids.sort();
        assert!(get_tokens_kid_response
            .kids
            .binary_search(&token_key_id.to_string())
            .is_ok());
    }

    async fn test_revoke_token(client: &Client, access_token: &str) {
        client
            .revoke_token(
                client.get_channel_id(),
                client.get_channel_secret(),
                access_token,
            )
            .await
            .unwrap();
    }

    async fn test_verify_token_v2(client: &Client, access_token: &str) {
        let verify_token_response = client.verify_token_v2(access_token).await.unwrap();
        assert_eq!(verify_token_response.client_id, client.get_channel_id());
    }

    async fn test_verify_token_v2_error(client: &Client, access_token: &str) {
        let verify_token_response = client.verify_token_v2(access_token).await;
        assert!(verify_token_response.is_err());
    }

    async fn test_revoke_token_v2(client: &Client, access_token: &str) {
        client.revoke_token_v2(access_token).await.unwrap();
    }

    #[actix_web::test]
    async fn test_token_v2_1() {
        let kid = env::var("JWT_TEST_KID").unwrap();
        let private_key = env::var("JWT_PRIVATE_KEY").unwrap();

        let client = create_client();
        let jwt = jwt::create_jwt(&kid, client.get_channel_id(), &private_key).unwrap();

        let issue_token_response = client.issue_token(&jwt).await.unwrap();
        test_verify_token(&client, &issue_token_response.access_token).await;

        test_get_tokens_kid(&kid, &private_key, &client, &issue_token_response.key_id).await;

        test_revoke_token(&client, &issue_token_response.access_token).await;

        test_verify_token_error(&client, &issue_token_response.access_token).await;
    }

    #[actix_web::test]
    async fn test_token_v2() {
        let client = create_client();

        let issue_token_v2_response = client
            .issue_token_v2(client.get_channel_id(), client.get_channel_secret())
            .await
            .unwrap();

        test_verify_token_v2(&client, &issue_token_v2_response.access_token).await;

        test_revoke_token_v2(&client, &issue_token_v2_response.access_token).await;

        test_verify_token_v2_error(&client, &issue_token_v2_response.access_token).await;
    }
} */
